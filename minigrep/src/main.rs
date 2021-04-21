use std::{process, env};
use minigrep::helpers;
mod tests;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = helpers::Config::new(&args[1..]).unwrap_or_else(|err| {
        eprintln!("Error while parsing arguments: {}", err);
        process::exit(1)
    });
    println!("Serching for {} in {}", config.query(), config.filename());
    if let Err(err) = helpers::run(&config) {
        eprintln!("Error while parsing file: {:?}", err);
        process::exit(1)
    }
}

