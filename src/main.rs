#![allow(dead_code, unused)]

use chrono::{NaiveDateTime, ParseError};
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
    started: NaiveDateTime,
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

// should return a vector of Songs
fn parse_logs(logfile: &Log) -> Result<(), ParseError> {
    let rows: Vec<&str> = logfile.contents.split("\r\n").collect();

    // dbg!(&rows);
    for row in rows {
        if row.contains("started") || row.contains("stopped") {
            // row only contains session start time, requires different logic to parse
        } else if row == "" {
            // ingore empty rows
        } else {
            let mut parts: Vec<&str> = row.split(" - ").collect();
            let mut parts_iter = parts.iter();

            let datepart = parts_iter.next().unwrap();
            let timepart = parts_iter.next().unwrap();
            let artistpart = remove_formatting(parts_iter.next().unwrap());
            let namepart = remove_formatting(parts_iter.next().unwrap());
            let lengthpart = parts_iter.next().unwrap();

            let datetimepartstr = format!("{datepart} {timepart}");
            let datetimepart =
                NaiveDateTime::parse_from_str(&datetimepartstr, "%d-%m-%Y %H:%M:%S").unwrap();

            dbg!(artistpart, namepart, lengthpart);
        }
    }

    Ok(())
}

fn remove_formatting(value: &&str) -> String {
    let mut chars = value.chars();

    chars.next();
    chars.next_back();

    chars.as_str().to_string()
}
