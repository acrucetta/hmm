pub mod thought;

use chrono::prelude::*;
use clap::{arg, command, Command};
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};

#[derive(Debug, Deserialize, Serialize)]
struct Row {
    id: u32,
    timestamp: String,
    message: String,
    tags: String,
}

fn get_next_id(file_path: &String) -> u32 {
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

fn add_thought(thought: &String, file_path: &String) -> Result<(), Box<dyn std::error::Error>>{
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
    let id = get_next_id(&file_path);
    let timestamp = get_current_timestamp();
    let message = thought.trim().to_string();

    // Append the thought to the CSV file
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .unwrap();

    let mut writer = csv::Writer::from_writer(&file);

    // Write the header row if the file is empty
    if id == 1 {
        writer.write_record(&["id", "timestamp", "message", "tags"])?;
    }

    writer.write_record(&[id.to_string(), timestamp, message, tags])?;

    Ok(())

}

fn list_thoughts(file_path: &String) -> Result<(), Box<dyn std::error::Error>> {
    if !std::path::Path::new(&file_path).exists() {
        println!("No thoughts found, add one with `hmm add (thought)`");
        return Ok(());
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
    Ok(())
}

pub fn remove_thought(id: &String, file_path: &String) -> Result<(), Box<dyn std::error::Error>> {
    // Open the CSV file and parse it to a Rust data structure
    let mut rdr = csv::Reader::from_path(&file_path).unwrap();
    let rows = rdr.deserialize::<Row>();
    let mut data: Vec<Row> = rows.filter_map(Result::ok).collect();

    // Filter out the row with the ID we want to remove
    data.retain(|row| row.id.to_string() != *id);
    
    // Write the updated data structure back to the CSV file
    let mut wtr = csv::Writer::from_path(&file_path).unwrap();
    for row in &data {
        wtr.serialize(row).unwrap();
    }

    Ok(())
}

fn get_output_dir() -> String {
    // Get .env from the path of the github repository
    // TODO: Change this later.
    const DOTENV_PATH: &str = "/Users/andrescrucettanieto/Library/CloudStorage/OneDrive-WaltzHealth/Documents/Code/hmm/.env";
    dotenv::from_path(DOTENV_PATH).ok();
    match env::var("HMM_OUTPUT_DIR") {
        Ok(val) => return val,
        Err(_) => println!("HMM_OUTPUT_DIR not set, using current directory"),
    }
    let curr_dir = ".";
    return curr_dir.to_string();
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

    let file_path = format!("{}/thoughts.csv", get_output_dir());

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let thought = sub_matches.get_one::<String>("THOUGHT").unwrap();
            match add_thought(&thought, &file_path) {
                Ok(_) => println!("Thought added!"),
                Err(e) => println!("Error adding thought: {}", e),
            }
        }
        Some(("ls", _sub_matches)) => {
            match list_thoughts(&file_path) {
                Ok(_) => (),
                Err(e) => println!("Error listing thoughts: {}", e),
            }
        }
        Some(("rm", sub_matches)) => {
            let id = sub_matches.get_one::<String>("THOGUHT_ID").unwrap();
            match remove_thought(&id, &file_path) {
                Ok(_) => println!("Thought removed!"),
                Err(e) => println!("Error removing thought: {}", e),
            }
        }
        Some(("clear", _sub_matches)) => {
            match remove_all_thoughts(&file_path) {
                Ok(_) => println!("All thoughts removed!"),
                Err(e) => println!("Error removing all thoughts: {}", e),
            }
        }
        _ => println!("No subcommand was used"),
    }
}

fn remove_all_thoughts(file_path: &String) -> Result<(), Box<dyn std::error::Error>> {
    // Confirm that the user wants to delete all thoughts
    println!("Are you sure you want to delete all thoughts? (y/n):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim() != "y" {
        println!("Thoughts not deleted");
        return Ok(());
    }

    // Deletes all the rows in the CSV file
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)
        .unwrap();

    // Write the header row
    writeln!(file, "id,timestamp,message,tags").unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_thought() {
        // Create a temporary CSV file for testing
        let file_path = "test_remove_thought.csv";
        let mut wtr = csv::Writer::from_path(file_path).unwrap();
        wtr.write_record(&["id", "text"]).unwrap();
        wtr.write_record(&["1", "First thought"]).unwrap();
        wtr.write_record(&["2", "Second thought"]).unwrap();
        wtr.write_record(&["3", "Third thought"]).unwrap();
        wtr.flush().unwrap();

        // Remove the second thought from the CSV file
        remove_thought(&"2".to_string());

        // Read the CSV file and check that the second thought was removed
        let mut rdr = csv::Reader::from_path(file_path).unwrap();
        let rows = rdr.deserialize::<Row>();
        let data: Vec<Row> = rows.filter_map(Result::ok).collect();
        assert_eq!(data.len(), 2);
        assert_eq!(data[0].id, 1);
        assert_eq!(data[0].message, "First thought");
        assert_eq!(data[1].id, 3);
        assert_eq!(data[1].message, "Third thought");

        // Remove the CSV file
        std::fs::remove_file(file_path).unwrap();

    }
}
