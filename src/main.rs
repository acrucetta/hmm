mod thought;
mod cli;

use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use chrono::prelude::*;
use clap::ArgMatches;

fn get_next_id() -> u32 {
    let file = File::open("thoughts.csv");
    match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            let mut id = 0;
            for line in reader.lines() {
                let line = line.unwrap();
                let split: Vec<&str> = line.split(",").collect();
                let current_id = split[0].parse::<u32>().unwrap();
                if current_id > id {
                    id = current_id;
                }
            }
            id + 1
        }
        Err(_) => 1,
    }
}

fn get_current_timestamp() -> String {
    let utc: DateTime<Utc> = Utc::now();
    utc.to_rfc3339()
}

fn add_thought(tags: Option<&str>) {
    // Prompt the user for a message
    println!("Enter your thought:");
    let mut message = String::new();
    std::io::stdin().read_line(&mut message).unwrap();

    // Prompt the user for tags (optional)
    let tags = match tags {
        Some(t) => t.trim().to_string(),
        None => {
            println!("Enter tags (optional):");
            let mut tags = String::new();
            std::io::stdin().read_line(&mut tags).unwrap();
            tags.trim().to_string()
        }
    };

    // Generate a new ID and timestamp
    let id = get_next_id();
    let timestamp = get_current_timestamp();

    // Create a new thought and append it to the CSV file
    let thought = thought::Thought {
        id,
        timestamp,
        message,
        tags: if tags.trim().is_empty() {
            None
        } else {
            Some(tags.trim().to_string())
        },
    };
    let mut writer = csv::Writer::from_path("thoughts.csv").unwrap();
    writer.serialize(thought).unwrap();
}

fn list_thoughts() {
    let mut reader = csv::Reader::from_path("thoughts.csv").unwrap();
    for result in reader.deserialize::<thought::Thought>() {
        let thought = result.unwrap();
        if let Some(tags) = thought.tags {
            println!("{} {} {} ({})", thought.id, thought.timestamp, thought.message, tags);
        } else {
            println!("{} {} {}", thought.id, thought.timestamp, thought.message);
        }
    }
}

fn remove_thought(id: u32) {
    let file = File::open("thoughts.csv").unwrap();
    let reader = BufReader::new(file);
    let mut writer = csv::Writer::from_path("temp.csv").unwrap();
    for line in reader.lines() {
        let line = line.unwrap();
        let split: Vec<&str> = line.split(",").collect();
        let current_id = split[0].parse::<u32>().unwrap();
        if current_id != id {
            writer.write(line.as_bytes()).unwrap();
            writer.write(b"\n").unwrap();
        }
    }
    std::fs::rename("temp.csv", "thoughts.csv").unwrap();
}

fn main() {
    let matches = cli::build_cli().get_matches();
    match matches.subcommand() {
        ("+", Some(matches)) => {
            let tags = matches.value_of("tags");
            add_thought(tags);
        }
        ("ls", Some(_)) => list_thoughts(),
        ("rm", Some(matches)) => {
            let id = matches.value_of("id").unwrap().parse::<u32>().unwrap();
            remove_thought(id)
        }
        _ => println!("Please provide a valid command"),
    }
}
    