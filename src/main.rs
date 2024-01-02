

fn main() {
    let (query, file_path) = minigrep::parse_args();
    println!("Searching for {} in file {}", query, file_path);

    let contents = minigrep::read_file(&file_path);
}
