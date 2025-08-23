use std::io::{self, Write};
use std::path::Path;
use std::thread;
use std::thread::sleep;
use std::time::{Duration};

use chrono::{Local, Timelike};
use rodio::Decoder;
use std::fs::File;

use std::env;
use std::fs;

fn main() {

    let stdin = io::stdin();
    let input_minutes = &mut String::new();
    let mut num_of_minutes: i64 = 0;

    print!("Minute >> ");
    let _ = io::stdout().flush();

    match stdin.read_line(input_minutes) {
        Ok(_) => {
            let num_of_minutes_parsed = input_minutes.trim().parse::<i64>();
            match num_of_minutes_parsed {
                Ok(_) => {
                    num_of_minutes = num_of_minutes_parsed.unwrap();
                }
                Err(_) => println!("Not a valid value, expect a number of minutes"),
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    let now = Local::now();
    let finish_time = now + chrono::Duration::minutes(num_of_minutes);

    println!(
        "Started at {:02}:{:02}, will finish at {:02}:{:02}",
        now.hour(),
        now.minute(),
        finish_time.hour(),
        finish_time.minute()
    );

    let handle = thread::spawn(move || {
        for i in (1..num_of_minutes + 1).rev() {
            print!("\r\x1B[2KTIME LEFT: {} minute", i);
            io::stdout().flush().unwrap();
            sleep(Duration::from_secs(1));
        }
    });

    handle.join().unwrap();

    // When it reaches here, start the notification by playing a song
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    let sink = rodio::Sink::connect_new(&stream_handle.mixer());

    let mut path = env::current_exe().unwrap();

    let project_root = env!("CARGO_MANIFEST_DIR");
    let src = format!("{}/Kevin_MacLeod_-_Canon_in_D_Major(chosic.com).mp3", project_root);
    let debug_target = "target/debug/Kevin_MacLeod_-_Canon_in_D_Major(chosic.com).mp3";
    let release_target = "target/release/Kevin_MacLeod_-_Canon_in_D_Major(chosic.com).mp3";


    println!(""); // print a new line
    for target in [debug_target, release_target] {
        let target_path = Path::new(target);

        if let Some(parent) = target_path.parent() {
            if !parent.exists() {
                if let Err(e) = fs::create_dir_all(parent) {
                    eprintln!("Failed to create the path: {:?}", e);
                    continue;
                }
            }
        }

        if !target_path.exists() {
            if let Err(e) = fs::copy(&src, target) {
                eprintln!("Failed to copy to {}: {}", target, e);
            } else {
                println!("Copied to {}", target);
            }
        }
    }

    path.pop(); // remove binary file name
    path.push(src);

    let file = File::open(path).unwrap();
    let source = Decoder::try_from(file).unwrap();
    sink.append(source);
    // stream_handle.mixer().add(source);
    sink.sleep_until_end();
    // std::thread::sleep(std::time::Duration::from_secs(5));
}
