#![allow(dead_code, unused)]

use std::env;
use std::fs;
use std::fs::{DirEntry, File, ReadDir};
use std::io;
use std::io::Read;
use std::time;
use std::time::Duration;

#[derive(Debug)]
struct Log {
    filepath: DirEntry,
    contents: String,
}

#[derive(Debug)]
struct Song {
    artist: String,
    name: String,
    length: Duration,
    started: DateTime,
}

fn main() -> Result<(), io::Error> {
    let logs = load_files("/mnt/c/Users/sand/AppData/Roaming/WACUP/Logs/")?;

    parse_logs(&logs[0]);

    Ok(())
    // let contents = fs::read_to_string(file_path).expect("should have read file");
}

fn load_files(path: &str) -> Result<Vec<Log>, io::Error> {
    let mut files: Vec<Log> = Vec::new();
    let paths = fs::read_dir(path).unwrap().filter_map(|e| e.ok());

    for path in paths {
        let mut file = File::open(path.path())?;
        let mut content = String::new();

        file.read_to_string(&mut content)?;

        files.push(Log {
            filepath: path,
            contents: content,
        });
    }

    Ok(files)
}

fn parse_logs(logfile: &Log) {
    let rows: Vec<&str> = logfile.contents.split("\n").collect();

    dbg!(rows);
}
