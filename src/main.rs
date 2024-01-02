use std::collections::VecDeque;

use minigrep::Config;

fn main() {
    let args = std::env::args().collect::<VecDeque<String>>();

    let config = Config::new(args);
    println!(
        "Searching for {} in file {}",
        config.query, config.file_path
    );

    let contents = minigrep::read_file(&config.file_path);
}
