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

use colored::Colorize;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{display::{Display, LETTERS}, game::cell::RevealResult, config::Config};

use self::{cell::{Cells, CellType}, position::Position};

pub mod cell;
pub mod position;

enum EventType {
    GameOver,
    Win,
    Error(String),
}

pub struct Game<'a> {
    map: Cells<'a>,
    display: Display<'a>,
    event: Option<EventType>,
    mines_left: i32,
    config: &'a Config,
}

impl<'a> Game<'a> {
    pub fn new(config: &'a Config) -> Self {
        assert!(config.mines <= i32::MAX as u32);
        Self {
            display: Display::new(&config),
            map: Cells::new(&config),
            event: None,
            mines_left: config.mines as i32,
            config,
        }
    }

    pub fn run(&mut self) {
        // game loop
        loop {
            self.display.clear_screen();
            self.display.render_board(&self.map);

            // handle event
            if let Some(etype) = &self.event {
                match etype {
                    EventType::Error(msg) => println!("[ERR] {}", msg.red()),
                    EventType::Win => {
                        println!("{}", "!!! YOU WON !!!".bright_green());
                        break;
                    },
                    EventType::GameOver => {
                        println!("{}", "!!! GAME OVER !!!".red());
                        break;
                    }
                }
                self.event = None;
            }

            let input = self.display.get_input(self.mines_left).to_lowercase();
            
            if input.is_empty() {
                self.event = Some(EventType::Error("enter command".to_string()));
                continue;
            }

            let mut args_iter = input.split_whitespace();

            match args_iter.next().unwrap() {
                "quit" | "exit" | "q" => break,
                "flag" | "f" => {
                    let pos_str = args_iter.next();

                    if pos_str.is_none() {
                        self.event = Some(EventType::Error("pass coordinates as argument".to_string()));
                        continue;
                    }

                    let pos = match self.get_pos_from_str(pos_str.unwrap()) {
                        Ok(pos) => pos,
                        Err(msg) => {
                            self.event = Some(EventType::Error(msg));
                            continue;
                        }
                    };
                    
                    self.flag_cell(pos);
                },
                "r" => {
                    self.reveal_possible();
                },
                "help" | "?" | "h" => {
                    self.display.print_help();
                },
                _ => {
                    lazy_static! {
                        // fxy is shortcut for f xy
                        static ref FLAG_RE: Regex = Regex::new("^f[a-z][0-9]+$").unwrap();
                    }

                    let mut flag = false;
                    let mut to_parse = input.as_str();
                    
                    // check flag shortcut
                    if FLAG_RE.is_match(&input) {
                        flag = true;
                        to_parse = &input[1..];
                    }

                    let pos = match self.get_pos_from_str(&to_parse) {
                        Ok(pos) => pos,
                        Err(msg) => {
                            self.event = Some(EventType::Error(msg));
                            continue;
                        }
                    };

                    if flag {
                        self.flag_cell(pos);
                        continue;
                    }

                    // if there is an event to handle continue
                    if self.guess_cell(pos, &input) {
                        continue;
                    }
                }
            }
        }
    }
    
    // guess cell
    // returns:
    //  - true - if there is event to process
    //  - false - otherwise
    fn guess_cell(&mut self, pos: Position, input: &str) -> bool {
        if self.map.idx(pos).flag {
            self.event = Some(EventType::Error(format!("there is a flag on {}", &input)));
            return true;
        }

        match self.map.reveal(pos) {
            RevealResult::Normal => {},
            RevealResult::Mine => {
                self.map.reveal_all();
                self.event = Some(EventType::GameOver);
                return true;
            },
            RevealResult::Win => {
                self.map.reveal_all();
                self.event = Some(EventType::Win);
                return true;
            }
        }

        false
    }

    fn reveal_possible(&mut self) {
        let mut to_reveal = Vec::new();
        for x in 0..self.config.width {
            for y in 0..self.config.height {
                let pos = Position::new(x, y);
                let cell = self.map.idx(pos);

                if cell.hidden {
                    continue;
                }

                if let CellType::Number(_) = cell.ctype {
                    to_reveal.push(pos);
                }
            }
        }

        for pos in to_reveal {
            // no need for input because cell is never a flag
            if self.guess_cell(pos, "") {
                break;
            }
        }
    }

    fn flag_cell(&mut self, pos: Position) {
        if !self.map.idx(pos).hidden {
            self.event = Some(EventType::Error("cell is not hidden".to_string()));
            return;
        }

        self.map.flag_cell(pos);

        if self.map.idx(pos).flag {
            self.mines_left -= 1;
        } else {
            self.mines_left += 1;
        }
    }

    // get xy from input. A10 -> x=0 y=10
    fn get_pos_from_str(&self, input: &str) -> Result<Position, String> {
        lazy_static! {
            static ref RE: Regex = Regex::new("^[a-z][0-9]+$").unwrap();
        }

        // validate coordinates
        if !RE.is_match(&input) {
            return Err("bad coordinates".to_string());
        }

        let x = input.chars().next().unwrap();
        let x = LETTERS.to_lowercase().chars().position(|e| e == x).unwrap();
        let y = input[1..].parse().unwrap();

        if x > self.config.width - 1 {
            return Err("x is too big".to_string());
        }
        if y > self.config.height - 1 {
            return Err("y is too big".to_string());
        }

        Ok(Position::new(x, y))
    }
}

