use std::{env, collections::VecDeque, fs};

pub struct Config {
  query: String,
  file_path: String
}

pub fn parse_args() -> (String, String) {
  let mut args: VecDeque<String> = env::args().collect();

  args.pop_front().expect("panic");
  let query = args.pop_front().expect("no query");
  let file_path = args.remove(0).expect("no file path");

  (query, file_path)
}

pub fn read_file(file_path: &String) -> String {
  fs::read_to_string(file_path).expect("read file failed")
}
