use std::{collections::VecDeque, env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(mut args: VecDeque<String>) -> Config {
        args.pop_front().expect("panic");
        let query = args.pop_front().expect("no query");
        let file_path = args.remove(0).expect("no file path");

        Config { query, file_path }
    }
}

pub fn read_file(file_path: &String) -> String {
    fs::read_to_string(file_path).expect("read file failed")
}
