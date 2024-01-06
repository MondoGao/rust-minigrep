use std::{collections::VecDeque, process};

use minigrep::Config;

fn main() {
    let args = std::env::args().collect::<VecDeque<String>>();

    let config = Config::new(args).expect("parse config failed");
    println!(
        "Searching for {} in file {}",
        config.query, config.file_path
    );

    if let Err(e) = minigrep::run(config) {
        println!("Error: {e}");
        process::exit(1);
    }
}
