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

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "world";
        let contents = "\
        hello world
        你好";

        assert_eq!(vec!["hello world"], search(query, contents));
    }
}
