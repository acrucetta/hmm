mod thought;

use chrono::prelude::*;
use clap::{arg, command, Command};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};

fn get_next_id() -> u32 {
    let output_dir = get_output_dir();
    let file_path = format!("{}/thoughts.csv", output_dir);
    let file = File::open(file_path);

    // We want to return 1 if the file doesn't exist,
    // otherwise we want to return the next ID
    // To get the next ID we search the last line of the file in the ID column
    // and increment it by 1
    match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            let mut last_line = String::new();
            for line in reader.lines() {
                last_line = line.unwrap();
            }
            let split: Vec<&str> = last_line.split(",").collect();
            if split.len() == 0 {
                return 1;
            }
            // If we're returning the header row, return 1
            if split[0] == "id" {
                return 1;
            }
            let last_id = split[0].parse::<u32>().unwrap();
            last_id + 1
        }
        Err(_) => 1,
    }
}

fn get_current_timestamp() -> String {
    let utc: DateTime<Utc> = Utc::now();
    // Format the timestamp as YYYY-MM-DD
    utc.format("%Y-%m-%d").to_string()
}

fn add_thought(thought: &String) {
    let output_dir = get_output_dir();
    let file_path = format!("{}/thoughts.csv", output_dir);
    // Prompt the user for tags (optional)
    let tags = {
        println!("Enter tags (optional):");
        let mut tags = String::new();
        std::io::stdin().read_line(&mut tags).unwrap();
        tags.trim().to_string()
    };

    // Generate a new ID and timestamp
    let id = get_next_id();
    let timestamp = get_current_timestamp();
    let message = thought.trim().to_string();

    // Create a new thought and append it to the CSV file
    let thought = thought::Thought {
        id,
        timestamp,
        message,
        tags,
    };

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .unwrap();
    if file.metadata().unwrap().len() == 0 {
        writeln!(file, "id,timestamp,message,tags").unwrap();
    }
    writeln!(
        file,
        "{},{},{},{}",
        thought.id, thought.timestamp, thought.message, thought.tags
    )
    .unwrap();
}

fn list_thoughts() {
    let output_dir = get_output_dir();
    let file_path = format!("{}/thoughts.csv", output_dir);
    if !std::path::Path::new(&file_path).exists() {
        println!("No thoughts found, add one with `hmm add (thought)`");
        return;
    } 
    let mut reader = csv::Reader::from_path(file_path).unwrap();
    println!("ID Timestamp Thought Tags");
    for result in reader.deserialize::<thought::Thought>() {
        let thought = result.unwrap();
        println!(
            "{} {} `{}` {}",
            thought.id, thought.timestamp, thought.message, thought.tags
        );
    }
}

fn remove_thought(id: &String) {

    let output_dir = get_output_dir();
    let file_path = format!("{}/thoughts.csv", output_dir);
    let temp_file_path = format!("{}/temp.csv", output_dir);
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let writer_file = File::create(temp_file_path).unwrap();
    
    let mut writer = csv::Writer::from_writer(BufWriter::new(writer_file));

    for line in reader.lines() {
        let line = line.unwrap();
        let split: Vec<&str> = line.split(",").collect();
        let current_id = split[0];
        if current_id != *id {
            writer.write_record(split).unwrap();
        }
    }
    std::fs::rename("temp.csv", "thoughts.csv").unwrap();
}

fn get_output_dir() -> String {
    // Get the output directory from the environment variable
    let output_dir = std::env::var("HMM_OUTPUT_DIR").unwrap_or(".".to_string());
    return output_dir;
}

fn main() { 
    let matches = command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Add a new thought")
                .arg(arg!([THOUGHT]))
                .arg_required_else_help(true),
        )
        .subcommand(Command::new("ls").about("List all thoughts"))
        .subcommand(
            Command::new("rm")
                .about("Remove a thought")
                .arg(arg!([THOGUHT_ID]))
                .arg_required_else_help(true),
        )
        .subcommand(Command::new("clear").about("Remove all thoughts"))
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let thought = sub_matches.get_one::<String>("THOUGHT").unwrap();
            add_thought(thought);
        }
        Some(("ls", _sub_matches)) => list_thoughts(),
        Some(("rm", sub_matches)) => {
            let id = sub_matches.get_one::<String>("THOGUHT_ID").unwrap();
            remove_thought(id);
        }
        Some(("clear", _sub_matches)) => {
            remove_all_thoughts();
        }
        _ => println!("No subcommand was used"),
    }
}

fn remove_all_thoughts() -> () {
    let output_dir = get_output_dir();
    let file_path = format!("{}/thoughts.csv", output_dir);
    // Confirm that the user wants to delete all thoughts
    println!("Are you sure you want to delete all thoughts? (y/n):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim() != "y" {
        return;
    }

    // Deletes all the rows in the CSV file
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)
        .unwrap();

    // Write the header row
    writeln!(file, "id,timestamp,message,tags").unwrap();
}
