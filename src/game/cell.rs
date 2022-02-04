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

use super::position::Position;

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
        Self {
            hidden: true,
            flag: false,
            ctype: CellType::Mine
        }
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

        // all cells are mines by default
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
                if self.idx(Position::new(x, y)).ctype != CellType::Empty {
                    continue;
                }

                let mut mine_count = 0;

                macro_rules! idx_is_mine {
                    ($a:expr,$b:expr) =>
                        (self.idx(Position::new($a, $b)).ctype == CellType::Mine)
                }

                if x > 0 && idx_is_mine!(x-1, y) {
                    mine_count += 1;
                }
                if x < self.config.width - 1 && idx_is_mine!(x+1, y) {
                    mine_count += 1;
                }
                if y > 0 && idx_is_mine!(x, y-1) {
                    mine_count += 1;
                }
                if y < self.config.height - 1 && idx_is_mine!(x, y+1) {
                    mine_count += 1;
                }
                if y > 0 && x > 0 && idx_is_mine!(x-1, y-1) {
                    mine_count += 1;
                }
                if y > 0 && x < self.config.width - 1 && idx_is_mine!(x+1, y-1) {
                    mine_count += 1;
                }
                if y < self.config.height - 1 && x > 0 && idx_is_mine!(x-1, y+1) {
                    mine_count += 1;
                }
                if y < self.config.height - 1 && x < self.config.width - 1 && idx_is_mine!(x+1, y+1) {
                    mine_count += 1;
                }
                
                // change cell number to number of mines
                if mine_count > 0 {
                    self.idx_mut(Position::new(x, y)).ctype = CellType::Number(mine_count);
                }
            }
        }
    }

    pub fn reveal(&mut self, pos: Position) -> RevealResult {
        let cell = self.idx(pos);

        if !cell.hidden {
            return self.reveal_visible(pos);
        }

        if cell.ctype == CellType::Mine {
            return RevealResult::Mine;
        }

        self.reveal_r(pos);

        if self.check_win() {
            RevealResult::Win
        } else {
            RevealResult::Normal
        }
    }

    // recursive reveal
    fn reveal_r(&mut self, pos: Position) {
        let cell = self.idx(pos);

        if !cell.hidden {
            return;
        }

        if cell.flag {
            return;
        }

        if cell.ctype == CellType::Mine {
            return;
        }

        self.idx_mut(pos).hidden = false;

        if let CellType::Number(_) = cell.ctype {
            return;
        }

        let x = pos.x;
        let y = pos.y;

        macro_rules! reveal_r_idx {
            ($a:expr, $b:expr) => {
                self.reveal_r(Position::new($a, $b));
            }
        }

        if x > 0 {
            reveal_r_idx!(x-1, y);
        }
        if x < self.config.width - 1 {
            reveal_r_idx!(x+1, y);
        }
        if y > 0 {
            reveal_r_idx!(x, y-1);
        }
        if y < self.config.height - 1 {
            reveal_r_idx!(x, y+1);
        }
        if x > 0 && y > 0 {
            reveal_r_idx!(x-1, y-1);
        }
        if x < self.config.width - 1 && y > 0 {
            reveal_r_idx!(x+1, y-1);
        }
        if x < self.config.width - 1 && y < self.config.height - 1 {
            reveal_r_idx!(x+1, y+1);
        }
        if x > 0 && y < self.config.height - 1 {
            reveal_r_idx!(x-1, y+1);
        }
    }

    fn reveal_visible(&mut self, pos: Position) -> RevealResult {
        if !self.reveal_visible_r(pos) {
            return RevealResult::Mine;
        }
        
        if self.check_win() {
            RevealResult::Win
        } else {
            RevealResult::Normal
        }
    }

    // recursive visible reveal
    // returns false if mine was revealed
    fn reveal_visible_r(&mut self, pos: Position) -> bool {
        let cell = self.idx_mut(pos);

        if cell.flag {
            return true;
        }

        if cell.ctype == CellType::Mine {
            return false;
        }

        cell.hidden = false;

        macro_rules! idx_flag {
            ($a:expr,$b:expr) => (self.idx(Position::new($a, $b)).flag)
        }

        let x = pos.x;
        let y = pos.y;

        // count flags
        if let CellType::Number(num) = cell.ctype {
            let mut flag_count = 0;
            if x > 0 && idx_flag!(x-1, y) {
                flag_count += 1;
            }
            if x < self.config.width - 1 && idx_flag!(x+1, y) {
                flag_count += 1;
            }
            if y > 0 && idx_flag!(x, y-1) {
                flag_count += 1;
            }
            if y < self.config.height - 1 && idx_flag!(x, y+1) {
                flag_count += 1;
            }
            if x > 0 && y > 0 && idx_flag!(x-1, y-1) {
                flag_count += 1;
            }
            if x < self.config.width - 1 && y > 0 && idx_flag!(x+1, y-1) {
                flag_count += 1;
            }
            if x < self.config.width - 1 && y < self.config.height - 1 && idx_flag!(x+1, y+1) {
                flag_count += 1;
            }
            if x > 0 && y < self.config.height - 1 && idx_flag!(x-1, y+1) {
                flag_count += 1;
            }

            if flag_count != num {
                return true;
            }
        }

        macro_rules! reveal_r_idx {
            ($a:expr, $b:expr) => {
                if !self.reveal_visible_r(Position::new($a,$b)){return false;};
            }
        }

        macro_rules! idx_hidden {
            ($a:expr, $b:expr) => (self.idx(Position::new($a, $b)).hidden)
        }
        
        if x > 0 && idx_hidden!(x-1, y) {
            reveal_r_idx!(x-1, y);
        }
        if x < self.config.width - 1 && idx_hidden!(x+1, y) {
            reveal_r_idx!(x+1, y);
        }
        if y > 0 && idx_hidden!(x, y-1) {
            reveal_r_idx!(x, y-1);
        }
        if y < self.config.height - 1 && idx_hidden!(x, y+1) {
            reveal_r_idx!(x, y+1);
        }
        if x > 0 && y > 0 && idx_hidden!(x-1, y-1) {
            reveal_r_idx!(x-1, y-1);
        }
        if x < self.config.width - 1 && y > 0 && idx_hidden!(x+1, y-1) {
            reveal_r_idx!(x+1, y-1);
        }
        if x < self.config.width - 1 && y < self.config.height - 1 && idx_hidden!(x+1, y+1) {
            reveal_r_idx!(x+1, y+1);
        }
        if x > 0 && y < self.config.height - 1 && idx_hidden!(x-1, y+1) {
            reveal_r_idx!(x-1, y+1);
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
                let cell = self.idx_mut(Position::new(x, y));
                cell.hidden = false;
                cell.flag = false;
            }
        }
    }

    pub fn idx(&self, pos: Position) -> Cell {
        assert!(pos.x < self.config.width);
        assert!(pos.y < self.config.height);

        self.data[pos.y * self.config.width + pos.x]
    }

    pub fn idx_mut(&mut self, pos: Position) -> &mut Cell {
        assert!(pos.x < self.config.width);
        assert!(pos.y < self.config.height);

        &mut self.data[pos.y * self.config.width + pos.x]
    }

    pub fn flag_cell(&mut self, pos: Position) {
        let cell = self.idx_mut(pos);

        if !cell.hidden {
            return;
        }

        cell.flag = !cell.flag;
    }
}
