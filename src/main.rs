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

use clap::{App, arg, ArgMatches};
use minesweeper::{config::Config, game::Game};

fn main() {
    let config = Config::new(get_args());

    if !config.is_ok() {
        eprintln!("{}", config.unwrap_err());
        return;
    }
    let config = config.unwrap();

    let mut game = Game::new(&config);

    game.run()
}

fn get_args() -> ArgMatches {
    App::new("minesweeper")
        .author("rev1e")
        .about("tui minesweeper game implemented in rust")
        .version("0.1.0")
        .arg(
            arg!(-w --width <width> "Width")
                .required(false)
                .default_value("8")
            )
        .arg(
            arg!(-h --height <height> "Height")
                .required(false)
                .default_value("8")
            )
        .arg(
            arg!(-m --mines <mines> "Number of mines")
                .required(false)
                .default_value("10")
            )
        .get_matches()
}
