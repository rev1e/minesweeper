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

use crate::{WIDTH, HEIGHT};

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

pub struct Cells {
    data: [Cell; WIDTH * HEIGHT],
}

impl Cells {
    pub fn new(mine_num: usize) -> Self {
        let mut cells = Self {
            data: [Cell::new(); WIDTH * HEIGHT],
        };

        cells.generate_mines(mine_num);

        cells
    }

    fn generate_mines(&mut self, mine_num: usize) {
        assert!(mine_num <= WIDTH * HEIGHT);

        let mut mine_fields: Vec<usize> = (0..(WIDTH * HEIGHT)).collect();
        let mut rng = rand::thread_rng();

        for _ in 0..(WIDTH * HEIGHT - mine_num) {
            let idx = rng.gen_range(0..mine_fields.len());
            self.data[mine_fields[idx]].ctype = CellType::Empty;
            mine_fields.remove(idx);
        }

        self.generate_numbers();
    }

    fn generate_numbers(&mut self) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                if self.idx(x, y).ctype != CellType::Empty {
                    continue;
                }

                let mut mines_count = 0;

                if x > 0 && self.idx(x-1, y).ctype == CellType::Mine {
                    mines_count += 1;
                }
                if x < WIDTH - 1 && self.idx(x+1, y).ctype == CellType::Mine {
                    mines_count += 1;
                }
                if y > 0 && self.idx(x, y-1).ctype == CellType::Mine {
                    mines_count += 1;
                }
                if y < HEIGHT - 1 && self.idx(x, y+1).ctype == CellType::Mine {
                    mines_count += 1;
                }
                if y > 0 && x > 0 && self.idx(x-1, y-1).ctype == CellType::Mine {
                    mines_count += 1;
                }
                if y > 0 && x < WIDTH - 1 && self.idx(x+1, y-1).ctype == CellType::Mine {
                    mines_count += 1;
                }
                if y < HEIGHT - 1 && x > 0 && self.idx(x-1, y+1).ctype == CellType::Mine {
                    mines_count += 1;
                }
                if y < HEIGHT - 1 && x < WIDTH - 1 && self.idx(x+1, y+1).ctype == CellType::Mine {
                    mines_count += 1;
                }
                
                if mines_count > 0 {
                    self.idx_mut(x, y).ctype = CellType::Number(mines_count);
                }
            }
        }
    }

    pub fn reveal(&mut self, x: usize, y: usize) -> RevealResult {
        assert!(x < WIDTH);
        assert!(y < HEIGHT);

        let cell = self.idx(x, y);

        if !cell.hidden {
            return RevealResult::Normal;
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
        assert!(x < WIDTH);
        assert!(y < HEIGHT);

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
        if x < WIDTH - 1 {
            self.reveal_r(x+1, y);
        }
        if y > 0 {
            self.reveal_r(x, y-1);
        }
        if y < HEIGHT - 1 {
            self.reveal_r(x, y+1);
        }
        if x > 0 && y > 0 {
            self.reveal_r(x-1, y-1);
        }
        if x < WIDTH - 1 && y > 0 {
            self.reveal_r(x+1, y-1);
        }
        if x < WIDTH - 1 && y < HEIGHT - 1 {
            self.reveal_r(x+1, y+1);
        }
        if x > 0 && y < HEIGHT - 1 {
            self.reveal_r(x-1, y+1);
        }
    }

    fn check_win(&self) -> bool {
        for cell in self.data {
            if cell.hidden && cell.ctype != CellType::Mine {
                return false;
            }
        }
        true
    }
    pub fn reveal_all(&mut self) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let cell = self.idx_mut(x, y);
                cell.hidden = false;
                cell.flag = false;
            }
        }
    }

    pub fn idx(&self, x: usize, y: usize) -> Cell {
        self.data[y * WIDTH + x]
    }

    pub fn idx_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.data[y * WIDTH + x]
    }

    pub fn flag_cell(&mut self, x: usize, y: usize) {
        assert!(x < WIDTH);
        assert!(y < HEIGHT);

        let cell = self.idx_mut(x, y);

        if !cell.hidden {
            return;
        }

        cell.flag = !cell.flag;
    }
}
