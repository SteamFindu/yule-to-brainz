#![allow(dead_code, unused)]

use std::env;
use std::fs;

use glob::glob;

fn main() {
    let args: Vec<String> = env::args().collect();
    let files: Vec<&str> = load_filepaths(&args[1]);

    dbg!(&args);
    dbg!(&files);

    // let contents = fs::read_to_string(file_path).expect("should have read file");

    println!("Contents:\n{contents}");
}

fn load_filepaths(path: &str) {
    let mut filepaths: Vec<String> = Vec::new();
    let paths = fs::read_dir(path).unwrap();

    for entry in glob("{path}").expect("failed to read glob") {
        match entry {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => panic!("{:?}", e),
        }
    }
}
