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

use clap::ArgMatches;

use crate::display::LETTERS;

#[derive(Debug)]
pub struct Config {
    pub width: usize,
    pub height: usize,
    pub mines: u32,
}

impl Config {
    pub fn new(args: ArgMatches) -> Result<Self, String> {
        let width = args.value_of("width").unwrap().parse()
            .map_err(|_| {
            return "width: invalid number".to_string();
        })?;
        let height = args.value_of("height").unwrap().parse()
            .map_err(|_| {
            return "height: invalid number".to_string();
        })?;
        let mines = args.value_of("mines").unwrap().parse()
            .map_err(|_| {
            return "mines: invalid number".to_string();
        })?;

        if height < 2 {
            return Err("min height is 2".to_string());
        }

        if height > 30 {
            return Err("max height is 30".to_string());
        }

        if width < 2 {
            return Err("min width is 2".to_string());
        }

        if width > LETTERS.len() {
            return Err(format!("max width is {}", LETTERS.len()));
        }

        if mines < 1 {
            return Err("minimal number of mines is 1".to_string());
        }

        if mines as usize > (width * height) - 1 {
            return Err(format!("max number of mines is {}", (width * height) - 1));
        }

        Ok(Self { width, height, mines })
    }
}
