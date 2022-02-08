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

use std::io::{self, Write};

use colored::Colorize;

use crate::{game::{cell::{Cells, CellType}, position::Position}, config::Config};

pub const LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

// handles i/o
pub struct Display<'a> {
    config: &'a Config
}

impl<'a> Display<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            config
        }
    }

    pub fn clear_screen(&self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    pub fn get_input(&self, mines_left: i32) -> String {
        // print prompt
        print!("({} mines left) -> ", mines_left);
        io::stdout().lock().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // remove \n
        input.pop();

        input
    }

    pub fn render_board(&self, map: &Cells) {
        print!("     ");
        for x in 0..(self.config.width - 1) {
            print!("{} ", LETTERS.chars().nth(x).unwrap());
        }
        println!("{}", LETTERS.chars().nth(self.config.width - 1).unwrap());

        print!("     ");
        for _ in 0..(self.config.width * 2) {
            print!("-");
        }
        print!("\n");

        for y in 0..self.config.height {
            print!("{:02} | ", y);
            for x in 0..self.config.width {
                let cell = map.idx(Position::new(x, y));
                
                if cell.flag {
                    print!("{} ", "!".red());
                    continue;
                }

                if cell.hidden {
                    print!("# ");
                    continue;
                }

                match cell.ctype {
                    CellType::Mine => print!("{} ", "*".red()),
                    CellType::Empty => print!("{} ", ".".bright_black()),
                    CellType::Number(n) => {
                        if n < 3 {
                            print!("{} ", format!("{}", n).bright_green());
                        } else if n < 5 {
                            print!("{} ", format!("{}", n).yellow());
                        } else {
                            print!("{} ", format!("{}", n).bright_red());
                        }
                    }
                }
            }   
            println!("| {:02}", y);
        }

        print!("     ");
        for _ in 0..(self.config.width * 2) {
            print!("-");
        }
        print!("\n");

        print!("     ");
        for x in 0..(self.config.width - 1) {
            print!("{} ", LETTERS.chars().nth(x).unwrap());
        }
        println!("{}", LETTERS.chars().nth(self.config.width - 1).unwrap());
    }


    pub fn print_help(&self) {
        println!("Help:");
        println!("<pos> -> guess");
        println!("f <pos>, flag <pos>, f<pos> -> flag position");
        println!("r -> reveal all possible");
        println!("quit, exit, q -> exit game");
        println!("help, h, ? -> print this message");
        print!("Press enter to continue..");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut String::new()).unwrap();
    }
}
