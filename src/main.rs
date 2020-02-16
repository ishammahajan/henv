#[macro_use]
extern crate clap;
use clap::App;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use webbrowser;

fn run_env_change_script(matches: &str) {
    let path = Path::new("$HOME/.keyboard-env");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Could not modify {}: {}", display, why.description()),
        Ok(file) => file,
    };

    match file.write_all(matches.as_bytes()) {
        Err(why) => panic!("Could not write to {}: {}", display, why.description()),
        Ok(_) => println!("Environment changed to: {}", matches),
    }
}

fn run_refresh_script() {
    webbrowser::open("hammerspoon://refresh").unwrap();
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.occurrences_of("refresh") {
        0 => {}
        _ => {
            run_refresh_script();
            return;
        }
    }

    if let Some(matches) = matches.value_of("ENVIRONMENT") {
        run_env_change_script(matches);
        run_refresh_script();
        return;
    } else {
        println!("Please supply an environment to change to");
    }
}
