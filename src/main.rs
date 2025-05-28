use std::{env, process};

use lilgrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = lilgrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
