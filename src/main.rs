#![allow(dead_code, unreachable_code)]

use chrono::naive::serde::ts_seconds_option;
use chrono::{DateTime, Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::{DirEntry, File};
use std::io;
use std::io::Read;

#[derive(Debug)]
struct Log {
    filepath: DirEntry,
    contents: String,
    parsed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Song {
    artist: String,
    name: String,
    length: i32,
    #[serde(with = "ts_seconds_option")]
    started: Option<NaiveDateTime>,
}

fn main() -> Result<(), io::Error> {
    let logs = load_files("/mnt/c/Users/sand/AppData/Roaming/WACUP/Logs/")?;

    let songs = parse_logs(&logs[0]);

    let json = convert_to_json(&songs);

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
            parsed: false,
        });
    }

    Ok(files)
}

fn parse_logs(logfile: &Log) -> Vec<Song> {
    let rows: Vec<&str> = logfile.contents.split("\r\n").collect();

    // dbg!(&rows);
    let mut songs: Vec<Song> = Vec::new();

    let rows_clone = rows.clone();
    let mut next_row_pos: usize = 0;
    let timezone = Local::now().offset().clone();

    for row in rows {
        next_row_pos += 1; // next row
        if row.contains("<<<") {
            // row only contains session start time, requires different logic to parse
            /*
            let mut parts: Vec<&str> = row.split(" ").collect();

            let date = parts[3];
            let time = parts[5];

            let datetimepartstr = format!("{date} {time}");
            dbg!(&datetimepartstr);
            last_row_datetime =
                NaiveDateTime::parse_from_str(&datetimepartstr, "%d-%m-%Y %H:%M:%S").unwrap();
            */
        } else if row == "" {
            // ingore empty rows
        } else {
            let mut parts: Vec<&str> = row.split(" - ").collect();

            let _namestr = if parts.len() >= 6 {
                // something has been goofed up and it has split a part of the songs name.
                // this re-joins the suspected parts

                let extrasplit = parts.remove(4);
                let namestart = parts[3];
                let nameval = format!("{namestart} {extrasplit}");

                println!("song with name {nameval} has been super incorrecly tagged, fix it");
                continue;
                nameval
            } else {
                parts[3].to_string()
            };

            // remove continue from above function and uncomment this if you want to let trough badly formatted song titles
            //mem::replace(&mut parts[3], &namestr);

            // dbg!(&parts);
            let mut parts_iter = parts.iter();

            let datepart = parts_iter.next().unwrap();
            let timepart = parts_iter.next().unwrap();
            let artistpart = remove_formatting(parts_iter.next().unwrap());
            let namepart = remove_formatting(parts_iter.next().unwrap());
            let mut lengthpart = parts_iter.next().unwrap().split(":");

            if lengthpart.clone().count() > 2 {
                // just ignore songs with over 1 hour playtime
                println!("song {namepart} has over 1 hour playtime, not supported");
                continue;
            }

            let mut mins: i32 = lengthpart.next().unwrap().parse().unwrap();
            mins = mins * 60;
            let secs: i32 = lengthpart.next().unwrap().parse().unwrap();
            let lengthpartmsec = (mins + secs) * 1000;

            if lengthpartmsec == 0 {
                println!("song {namepart} has 0 length?? fix please");
                continue;
            }

            // dbg!(&artistpart, &namepart);

            let datetimepartstr = format!("{datepart} {timepart} {timezone}");
            let datetimepart = DateTime::parse_from_str(&datetimepartstr, "%d-%m-%Y %H:%M:%S %z")
                .unwrap()
                .naive_utc();

            let next_row_datetimepartstr = if rows_clone[next_row_pos].contains("stopped") {
                let row_clone: Vec<&str> = rows_clone[next_row_pos].split(" ").collect();
                let next_row_date = row_clone[3];
                let next_row_time = row_clone[5];

                format!("{next_row_date} {next_row_time} {timezone}")
            } else {
                let row_clone: Vec<&str> = rows_clone[next_row_pos].split(" - ").collect();
                let next_row_date = row_clone[0];
                let next_row_time = row_clone[1];

                // dbg!(next_row_date, next_row_time);

                format!("{next_row_date} {next_row_time} {timezone}")
            };

            let next_row_datetime =
                DateTime::parse_from_str(&next_row_datetimepartstr, "%d-%m-%Y %H:%M:%S %z")
                    .unwrap()
                    .naive_utc();

            if !more_than_half_listened(datetimepart, next_row_datetime, lengthpartmsec) {
                // println!("Song {namepart} was not listened to for over 50%");
                continue;
            }

            let song = Song {
                artist: artistpart,
                name: namepart,
                length: lengthpartmsec,
                started: Some(datetimepart),
            };

            songs.push(song);
        }
    }

    songs
}

fn remove_formatting(value: &&str) -> String {
    let mut chars = value.chars();

    chars.next();
    chars.next_back();

    chars.as_str().to_string()
}

fn more_than_half_listened(
    this_datetime: NaiveDateTime,
    next_datetime: NaiveDateTime,
    length: i32,
) -> bool {
    let diff = next_datetime - this_datetime;

    // dbg!(this_datetime, next_datetime, length, diff);
    if diff.num_milliseconds() > (length / 2).into() {
        true
    } else {
        false
    }
}
