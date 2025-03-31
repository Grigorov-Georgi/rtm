use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::env;

fn main() {
    let file_path = "time.txt";

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    if contents.is_empty() {
        write_to_file(0, &mut file);
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
                println!("Added {} minutes. Total {}", num, contents_parsed)
            }
            Err(_) => println!("Invalid number format after 'a' command"),
        }
    } else if let Some(minutes_str) = command.strip_prefix("s") {
        match minutes_str.parse::<i32>() {
            Ok(num) => {
                contents_parsed = contents_parsed - num;
                write_to_file(contents_parsed, &mut file);
                println!("Removed {} minutes. Total {}", num, 1)
            }
            Err(_) => println!("Invalid number format after 's' command"),
        }
    } else if command == "h" {
        let hours = contents_parsed / 60;
        let min = contents_parsed % 60;
        println!("Total: {}h {}m", hours, min);
    } else {
        println!("Invalid input!");
    }
}

fn write_to_file(contents: i32, file: &mut File) {
    let _ = file.set_len(0);
    file.seek(SeekFrom::Start(0)).unwrap();
    let _ = file.write_all(format!("{}", contents).as_bytes());
}
