#![allow(dead_code, unused)]

use chrono::{NaiveDate, NaiveDateTime, ParseError};
use std::env;
use std::fs;
use std::fs::{DirEntry, File, ReadDir};
use std::io;
use std::io::Read;
use std::mem;

#[derive(Debug)]
struct Log {
    filepath: DirEntry,
    contents: String,
}

#[derive(Debug)]
struct Song {
    artist: String,
    name: String,
    length: i32,
    started: NaiveDateTime,
}

fn main() -> Result<(), io::Error> {
    let logs = load_files("/mnt/c/Users/sand/AppData/Roaming/WACUP/Logs/")?;

    // parse_logs(&logs[1]);

    dbg!(parse_logs(&logs[1]));

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

fn parse_logs(logfile: &Log) -> Vec<Song> {
    let rows: Vec<&str> = logfile.contents.split("\r\n").collect();

    dbg!(&rows);
    let mut songs: Vec<Song> = Vec::new();

    let mut last_row_datetime: NaiveDateTime = NaiveDate::from_ymd_opt(1970, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let rows_clone = rows.clone();
    let mut next_row_pos: usize = 0;

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

            let mut namestr = if parts.len() >= 6 {
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
            let lengthpartsec = mins + secs;

            if lengthpartsec == 0 {
                continue;
            }

            // dbg!(&artistpart, &namepart);

            let datetimepartstr = format!("{datepart} {timepart}");
            let datetimepart =
                NaiveDateTime::parse_from_str(&datetimepartstr, "%d-%m-%Y %H:%M:%S").unwrap();

            if !more_than_half_listened(last_row_datetime, datetimepart, lengthpartsec) {
                println!("Song {namepart} was not listened to for over 50%");
                continue;
            }

            if rows_clone[next_row_pos].contains("stopped") {
                let mut row_clone: Vec<&str> = row.split(" ").collect();
                let clone_date = row_clone[3];
                let clone_time = row_clone[5];

                let clone_datetimepartstr = format!("{datepart} {timepart}");
                let next_row_datetime =
                    NaiveDateTime::parse_from_str(&clone_datetimepartstr, "%d-%m-%Y %H:%M:%S")
                        .unwrap();

                if !more_than_half_listened(datetimepart, next_row_datetime, lengthpartsec) {
                    println!("Song {namepart} was not listened to for over 50%");
                    continue;
                }
            }

            let song = Song {
                artist: artistpart,
                name: namepart,
                length: lengthpartsec,
                started: datetimepart,
            };

            songs.push(song);
            last_row_datetime = datetimepart;
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
    last_datetime: NaiveDateTime,
    current_datetime: NaiveDateTime,
    length: i32,
) -> bool {
    let diff = current_datetime - last_datetime;

    // dbg!(last_datetime, current_datetime, length, diff);
    if diff.num_seconds() > (length / 2).into() {
        true
    } else {
        false
    }
}
