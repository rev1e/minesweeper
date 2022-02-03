/*
Copyright 2022 rev1e

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use rand::Rng;

use crate::config::Config;

pub enum RevealResult {
    Mine,
    Win,
    Normal,
}

#[derive(Debug,Copy,Clone,PartialEq)]
pub enum CellType {
    Mine,
    Number(u32),
    Empty,
}

#[derive(Copy,Debug,Clone)]
pub struct Cell {
    pub hidden: bool,
    pub flag: bool,
    pub ctype: CellType,
}

impl Cell {
    fn new() -> Self {
        Self { hidden: true, flag: false, ctype: CellType::Mine }
    }
}

pub struct Cells<'a> {
    data: Vec<Cell>,
    config: &'a Config,
}

impl<'a> Cells<'a> {
    pub fn new(config: &'a Config) -> Self {
        assert!(config.width > 1);
        assert!(config.height > 1);

        let mut cells = Self {
            data: vec![Cell::new(); config.width * config.height],
            config,
        };

        cells.generate_mines();

        cells
    }

    fn generate_mines(&mut self) {
        assert!(self.config.mines as usize <= self.config.width * self.config.height);

        let mut mine_fields: Vec<usize> =
            (0..(self.config.width * self.config.height)).collect();
        let mut rng = rand::thread_rng();

        for _ in 0..(self.config.width * self.config.height - self.config.mines as usize) {
            let idx = rng.gen_range(0..mine_fields.len());
            self.data[mine_fields[idx]].ctype = CellType::Empty;
            mine_fields.remove(idx);
        }

        self.generate_numbers();
    }

    fn generate_numbers(&mut self) {
        for x in 0..self.config.width {
            for y in 0..self.config.height {
                if self.idx(x, y).ctype != CellType::Empty {
                    continue;
                }

                let mut mines_count = 0;

                if x > 0 && self.idx(x-1, y).ctype == CellType::Mine {
                    mines_count += 1;
                }
                if x < self.config.width - 1 && self.idx(x+1, y).ctype == CellType::Mine {
                    mines_count += 1;
                }
                if y > 0 && self.idx(x, y-1).ctype == CellType::Mine {
                    mines_count += 1;
                }
                if y < self.config.height - 1 && self.idx(x, y+1).ctype == CellType::Mine {
                    mines_count += 1;
                }
                if y > 0 && x > 0 && self.idx(x-1, y-1).ctype == CellType::Mine {
                    mines_count += 1;
                }
                if y > 0 && x < self.config.width - 1 && self.idx(x+1, y-1).ctype == CellType::Mine {
                    mines_count += 1;
                }
                if y < self.config.height - 1 && x > 0 && self.idx(x-1, y+1).ctype == CellType::Mine {
                    mines_count += 1;
                }
                if y < self.config.height - 1 && x < self.config.width - 1 && self.idx(x+1, y+1).ctype == CellType::Mine {
                    mines_count += 1;
                }
                
                if mines_count > 0 {
                    self.idx_mut(x, y).ctype = CellType::Number(mines_count);
                }
            }
        }
    }

    pub fn reveal(&mut self, x: usize, y: usize) -> RevealResult {
        let cell = self.idx(x, y);

        if !cell.hidden {
            return self.reveal_visible(x, y);
        }

        if cell.ctype == CellType::Mine {
            return RevealResult::Mine;
        }

        self.reveal_r(x, y);

        if self.check_win() {
            RevealResult::Win
        } else {
            RevealResult::Normal
        }
    }

    fn reveal_r(&mut self, x: usize, y: usize) {
        let cell = self.idx(x, y);

        if !cell.hidden {
            return;
        }

        if cell.flag {
            return;
        }

        if cell.ctype == CellType::Mine {
            return;
        }

        self.idx_mut(x, y).hidden = false;

        if let CellType::Number(_) = cell.ctype {
            return;
        }

        if x > 0 {
            self.reveal_r(x-1, y);
        }
        if x < self.config.width - 1 {
            self.reveal_r(x+1, y);
        }
        if y > 0 {
            self.reveal_r(x, y-1);
        }
        if y < self.config.height - 1 {
            self.reveal_r(x, y+1);
        }
        if x > 0 && y > 0 {
            self.reveal_r(x-1, y-1);
        }
        if x < self.config.width - 1 && y > 0 {
            self.reveal_r(x+1, y-1);
        }
        if x < self.config.width - 1 && y < self.config.height - 1 {
            self.reveal_r(x+1, y+1);
        }
        if x > 0 && y < self.config.height - 1 {
            self.reveal_r(x-1, y+1);
        }
    }

    fn reveal_visible(&mut self, x: usize, y: usize) -> RevealResult {
        if !self.reveal_visible_r(x, y) {
            return RevealResult::Mine;
        }
        
        if self.check_win() {
            RevealResult::Win
        } else {
            RevealResult::Normal
        }
    }

    fn reveal_visible_r(&mut self, x: usize, y: usize) -> bool {
        let cell = self.idx_mut(x, y);

        if cell.flag {
            return true;
        }

        if cell.ctype == CellType::Mine {
            return false;
        }

        cell.hidden = false;

        if let CellType::Number(num) = cell.ctype {
            let mut flag_count = 0;
            if x > 0 && self.idx(x-1, y).flag {
                flag_count += 1;
            }
            if x < self.config.width - 1 && self.idx(x+1, y).flag {
                flag_count += 1;
            }
            if y > 0 && self.idx(x, y-1).flag {
                flag_count += 1;
            }
            if y < self.config.height - 1 && self.idx(x, y+1).flag {
                flag_count += 1;
            }
            if x > 0 && y > 0 && self.idx(x-1, y-1).flag {
                flag_count += 1;
            }
            if x < self.config.width - 1 && y > 0 && self.idx(x+1, y-1).flag {
                flag_count += 1;
            }
            if x < self.config.width - 1 && y < self.config.height - 1 && self.idx(x+1, y+1).flag {
                flag_count += 1;
            }
            if x > 0 && y < self.config.height - 1 && self.idx(x-1, y+1).flag {
                flag_count += 1;
            }

            if flag_count != num {
                return true;
            }
        }
        
        if x > 0 && self.idx(x-1, y).hidden {
            if !self.reveal_visible_r(x-1, y) { return false; };
        }
        if x < self.config.width - 1 && self.idx(x+1, y).hidden {
            if !self.reveal_visible_r(x+1, y) { return false; };
        }
        if y > 0 && self.idx(x, y-1).hidden {
            if !self.reveal_visible_r(x, y-1) { return false; };
        }
        if y < self.config.height - 1 && self.idx(x, y+1).hidden {
            if !self.reveal_visible_r(x, y+1) { return false; };
        }
        if x > 0 && y > 0 && self.idx(x-1, y-1).hidden {
            if !self.reveal_visible_r(x-1, y-1) { return false; };
        }
        if x < self.config.width - 1 && y > 0 && self.idx(x+1, y-1).hidden {
            if !self.reveal_visible_r(x+1, y-1) { return false; };
        }
        if x < self.config.width - 1 && y < self.config.height - 1 && self.idx(x+1, y+1).hidden {
            if !self.reveal_visible_r(x+1, y+1) { return false; };
        }
        if x > 0 && y < self.config.height - 1 && self.idx(x-1, y+1).hidden {
            if !self.reveal_visible_r(x-1, y+1) { return false; };
        }

        true
    }

    fn check_win(&self) -> bool {
        for cell in &self.data {
            if cell.hidden && cell.ctype != CellType::Mine {
                return false;
            }
        }
        true
    }
    pub fn reveal_all(&mut self) {
        for x in 0..self.config.width {
            for y in 0..self.config.height {
                let cell = self.idx_mut(x, y);
                cell.hidden = false;
                cell.flag = false;
            }
        }
    }

    pub fn idx(&self, x: usize, y: usize) -> Cell {
        assert!(x < self.config.width);
        assert!(y < self.config.height);

        self.data[y * self.config.width + x]
    }

    pub fn idx_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        assert!(x < self.config.width);
        assert!(y < self.config.height);

        &mut self.data[y * self.config.width + x]
    }

    pub fn flag_cell(&mut self, x: usize, y: usize) {
        let cell = self.idx_mut(x, y);

        if !cell.hidden {
            return;
        }

        cell.flag = !cell.flag;
    }
}
