// #![allow(dead_code, unreachable_code, unused_imports)]

use chrono::naive::serde::ts_seconds_option;
use chrono::{DateTime, Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::fs::{DirEntry, File};
use std::io::Read;
use std::io::{self};
use std::{env, fs};

#[derive(Debug)]
struct Log {
    filepath: DirEntry,
    contents: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Submission {
    #[serde(with = "ts_seconds_option")]
    listened_at: Option<NaiveDateTime>,
    track_metadata: Song,
}

#[derive(Serialize, Deserialize, Debug)]
struct Song {
    artist_name: String,
    track_name: String,
    duration_ms: i32,
}

#[derive(Debug)]
struct Options {
    filepath: String,
    usertoken: String,
    // TODO: add delete option
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    let options = Options {
        filepath: args[1].to_string(),
        usertoken: args[2].to_string(),
    };

    // dbg!(&options);

    let logs = load_files(options.filepath)?;

    for log in logs {
        let songs = parse_logs(&log);

        if songs.len() == 0 {
            println!(
                "log {} empty, deleting...",
                &log.filepath.path().to_str().unwrap()
            );
            fs::remove_file(&log.filepath.path()).expect("failed to delete log.");
            continue;
        }

        let songs_json = serde_json::to_string(&songs)?;

        let request_json = format!(
            "{{\"listen_type\": \"import\",\"payload\": {}}}",
            songs_json
        );

        // println!("{}", request_json);

        let clint = reqwest::blocking::Client::new();
        let res = clint
            .post("https://api.listenbrainz.org/1/submit-listens")
            .header("Content-type", "application/json")
            .header("Authorization", "Token ".to_owned() + &options.usertoken)
            .body(request_json)
            .send()
            .expect("error sending request");

        // println!("{}", res.status());
        // File::create("res.html")?.write_all(res.as_bytes())?;

        if res.status() == 200 {
            println!(
                "{} succesfully submitted, deleting...",
                &log.filepath.path().to_str().unwrap()
            );

            fs::remove_file(&log.filepath.path()).expect("failed to delete log.");
        } else {
            println!(
                "error submitting log {}, status code {}",
                &log.filepath.path().to_str().unwrap(),
                res.status()
            )
        }
    }

    Ok(())
}

fn load_files(path: String) -> Result<Vec<Log>, io::Error> {
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

fn parse_logs(logfile: &Log) -> Vec<Submission> {
    println!("Parsing log {0}", logfile.filepath.path().to_str().unwrap());
    let rows: Vec<&str> = logfile.contents.split("\r\n").collect();

    // dbg!(&rows);
    let mut submissions: Vec<Submission> = Vec::new();

    // make a clone of rows to compare playtime with the next line, otherwise it would be consumed
    // by the iterator
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
                // nameval
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

            let mut mins: i32 = match lengthpart.next() {
                None => {
                    println!("song {namepart} has broken time. please fix.");
                    continue;
                }
                Some(x) => match x.parse() {
                    Err(_err) => {
                        println!("song {namepart} has an incorrect time format. please fix.");
                        continue;
                    }
                    Ok(var) => var,
                },
            };
            mins = mins * 60;
            let secs: i32 = match lengthpart.next() {
                None => {
                    println!("song {namepart} has broken time. please fix.");
                    continue;
                }
                Some(x) => match x.parse() {
                    Err(_err) => {
                        println!("song {namepart} has an incorrect time format. please fix.");
                        continue;
                    }
                    Ok(var) => var,
                },
            };
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

            if rows_clone[next_row_pos] != "" {
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

                if !more_than_half_listened(datetimepart, next_row_datetime, lengthpartmsec)
                    || lengthpartmsec < 240000
                {
                    /* Listens should be submitted for tracks when the user has listened to half the track or 4 minutes of the track,
                     * whichever is lower. If the user hasn’t listened to 4 minutes or half the track,
                     * it doesn’t fully count as a listen and should not be submitted. */

                    // println!("Song {namepart} was not listened to for over 50%");
                    continue;
                }
            }

            let song = Song {
                artist_name: artistpart,
                track_name: namepart,
                duration_ms: lengthpartmsec,
            };

            let submission = Submission {
                listened_at: Some(datetimepart),
                track_metadata: song,
            };

            submissions.push(submission);
        }
    }

    submissions
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
