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

use crate::{display::{Display, LETTERS}, HEIGHT, WIDTH, game::cell::RevealResult, MINES};

use self::cell::Cells;

pub mod cell;

enum EventType {
    GameOver,
    Win,
    Error(String),
}

pub struct Game {
    map: Cells,
    display: Display,
    event: Option<EventType>,
    mines_left: usize,
}

impl Game {
    pub fn new() -> Self {
        Self {
            display: Display::new(),
            map: Cells::new(MINES),
            event: None,
            mines_left: MINES,
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

                    let (x, y) = match get_pos_from_str(pos_str.unwrap()) {
                        Ok(pos) => pos,
                        Err(msg) => {
                            self.event = Some(EventType::Error(msg));
                            continue;
                        }
                    };

                    self.map.flag_cell(x, y);

                    if self.map.idx(x, y).flag {
                        self.mines_left -= 1;
                    } else {
                        self.mines_left += 1;
                    }
                },
                _ => {
                    let (x, y) = match get_pos_from_str(&input) {
                        Ok(pos) => pos,
                        Err(msg) => {
                            self.event = Some(EventType::Error(msg));
                            continue;
                        }
                    };

                    self.guess_cell(x, y, &input);
                }
            }
        }
    }

    fn guess_cell(&mut self, x: usize, y: usize, pos_str: &str) {
        if self.map.idx(x, y).flag {
            self.event = Some(EventType::Error(format!("there is a flag on {}", pos_str)));
            return;
        }
        match self.map.reveal(x, y) {
            RevealResult::Normal => {},
            RevealResult::Mine => {
                self.map.reveal_all();
                self.event = Some(EventType::GameOver);
            },
            RevealResult::Win => {
                self.map.reveal_all();
                self.event = Some(EventType::Win);
            }
        }
    }
}

fn get_pos_from_str(input: &str) -> Result<(usize, usize), String> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^[a-z][0-9]+$").unwrap();
    }

    if !RE.is_match(&input) {
        return Err("bad coordinates".to_string());
    }

    let x = input.chars().next().unwrap();
    let x = LETTERS.to_lowercase().chars().position(|e| e == x).unwrap();
    let y = input[1..].parse().unwrap();

    if x > WIDTH - 1 {
        return Err("x is too big".to_string());
    }
    if y > HEIGHT - 1 {
        return Err("y is too big".to_string());
    }

    Ok((x, y))
}
