use std::{collections::VecDeque, error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(mut args: VecDeque<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        args.pop_front().expect("panic");
        let query = args.pop_front().expect("no query");
        let file_path = args.remove(0).expect("no file path");

        Ok(Config { query, file_path })
    }
}

pub fn read_file(file_path: &String) -> String {
    fs::read_to_string(file_path).expect("read file failed")
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = read_file(&config.file_path);
    Ok(())
}
