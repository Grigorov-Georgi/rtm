use std::env;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

fn main() {
    let file_path = get_file_path();

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    if contents.trim().is_empty() {
        contents = "0".to_string();
    }

    let mut contents_parsed: i32 = contents.parse().unwrap();

    let command = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("No command provided");
        std::process::exit(1);
    });

    if let Some(minutes_str) = command.strip_prefix("a") {
        match minutes_str.parse::<i32>() {
            Ok(num) => {
                contents_parsed = contents_parsed + num;
                write_to_file(contents_parsed, &mut file);
                println!("Added {} minutes. {}", num, get_total(contents_parsed))
            }
            Err(_) => println!("Invalid number format after 'a' command"),
        }
    } else if let Some(minutes_str) = command.strip_prefix("s") {
        match minutes_str.parse::<i32>() {
            Ok(num) => {
                contents_parsed = contents_parsed - num;
                write_to_file(contents_parsed, &mut file);
                println!("Removed {} minutes. {}", num, get_total(contents_parsed))
            }
            Err(_) => println!("Invalid number format after 's' command"),
        }
    } else if command == "h" {
        println!("{}", get_total(contents_parsed));
    } else {
        println!("Invalid input!");
    }
}

fn get_total(minutes: i32) -> String {
    let hours = get_hours(minutes);
    let min = get_minutes(minutes);
    format!("Total {}h {}m", hours, min)
}

fn get_hours(minutes: i32) -> i32 {
    minutes / 60
}

fn get_minutes(minutes: i32) -> i32 {
    minutes % 60
}

fn get_file_path() -> PathBuf {
    let mut path = env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."));

    path.push(".rtm");
    create_dir_all(&path).expect("Failed to create .rtm dir");
    path.push("time.txt");
    path
}

fn write_to_file(contents: i32, file: &mut File) {
    let _ = file.set_len(0);
    file.seek(SeekFrom::Start(0)).unwrap();
    let _ = file.write_all(format!("{}", contents).as_bytes());
}
