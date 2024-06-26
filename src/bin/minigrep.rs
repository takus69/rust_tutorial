extern crate rust_tutorial;

use std::env;
use std::process;

use rust_tutorial::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = rust_tutorial::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}

