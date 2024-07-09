use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, file_name) = parse_config(&args);

    println!("Searching for {query}");
    println!("In file {file_name}");

    let contents = fs::read_to_string(file_name).expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    // dbg!(&args);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}