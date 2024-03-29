mod config;
pub mod thought;

use chrono::prelude::*;
use clap::{arg, command, Command};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize)]
struct Row {
    id: u32,
    timestamp: String,
    message: String,
    tags: String,
}

impl Eq for Row {}

impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.timestamp == other.timestamp
            && self.message == other.message
            && self.tags == other.tags
    }
}

fn get_next_id(rows: &Vec<Row>) -> u32 {
    let mut id = 1;
    for row in rows {
        if row.id >= id {
            id = row.id + 1;
        }
    }
    id
}

fn get_current_timestamp() -> String {
    let utc: DateTime<Utc> = Utc::now();
    // Format the timestamp as YYYY-MM-DD
    utc.format("%Y-%m-%d").to_string()
}

fn add_thought(thought: &String, mut rows: Vec<Row>) -> Vec<Row> {
    // Prompt the user for tags (optional)
    let tags = {
        println!("Enter tags (optional):");
        let mut tags = String::new();
        std::io::stdin().read_line(&mut tags).unwrap();
        tags.trim().to_string()
    };

    // Generate a new ID and timestamp
    let id = get_next_id(&rows);
    let timestamp = get_current_timestamp();
    let message = thought.trim().to_string();

    rows.push(Row {
        id,
        timestamp,
        message,
        tags,
    });

    rows
}

fn list_thoughts(rows: &Vec<Row>, tag_filter: Option<&String>) {
    if rows.is_empty() {
        warn!("No thoughts found! Add one with 'hmm add <thought>'");
    }
    let filtered_rows = match tag_filter {
        Some(tag) => rows
            .iter()
            .filter(|row| row.tags.contains(tag))
            .collect::<Vec<&Row>>(),
        None => rows.iter().collect::<Vec<&Row>>(),
    };
    for row in filtered_rows {
        println!(
            "\x1b[36m\n#{}, {}, {}\x1b[0m\n{}\n{}",
            row.id,
            row.timestamp,
            row.tags,
            color_string("------------------------@_'-'", "blue"),
            color_string(row.message.as_str(), "cyan")
        );
    }
}

/// .
fn color_string(string: &str, color: &str) -> String {
    // We will color the string based ob the color
    // the user provides; we will support:
    match color {
        "red" => format!("\x1b[31m{}\x1b[0m", string),
        "green" => format!("\x1b[32m{}\x1b[0m", string),
        "yellow" => format!("\x1b[33m{}\x1b[0m", string),
        "blue" => format!("\x1b[34m{}\x1b[0m", string),
        "magenta" => format!("\x1b[35m{}\x1b[0m", string),
        "cyan" => format!("\x1b[36m{}\x1b[0m", string),
        "white" => format!("\x1b[37m{}\x1b[0m", string),
        _ => string.to_string(),
    }
}

fn remove_thought(id: &String, mut rows: Vec<Row>) -> Vec<Row> {
    let mut index = 0;
    for row in &rows {
        if row.id.to_string() == *id {
            rows.remove(index);
            break;
        }
        index += 1;
    }
    rows
}

fn main() {
    let config = config::load_config();
    let matches = command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Add a new thought")
                .arg(arg!([THOUGHT]))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("ls")
                .about("List all thoughts")
                .arg(arg!([TAG]))
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("rm")
                .about("Remove a thought")
                .arg(arg!([THOGUHT_ID]))
                .arg_required_else_help(true),
        )
        .subcommand(Command::new("clear").about("Remove all thoughts"))
        .get_matches();

    let file_path = format!("{}/thoughts.csv", config.output_dir);

    // Load the file into rows
    let mut rows = match load_file_into_rows(&file_path) {
        Ok(rows) => rows,
        Err(_) => Vec::new(),
    };

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let thought = sub_matches.get_one::<String>("THOUGHT").unwrap();
            rows = add_thought(&thought, rows);
        }
        Some(("ls", sub_matches)) => {
            let tag = match sub_matches.get_one::<String>("TAG") {
                Some(tag) => Some(tag),
                None => None,
            };
            list_thoughts(&rows, tag);
        }
        Some(("rm", sub_matches)) => {
            let id = sub_matches.get_one::<String>("THOGUHT_ID").unwrap();
            rows = remove_thought(&id, rows);
        }
        Some(("clear", _sub_matches)) => {
            rows = remove_all_thoughts(rows);
        }
        _ => warn!("No subcommand was used"),
    }

    // Save the rows to the file
    match save_rows_to_file(&file_path, &rows) {
        Ok(_) => info!("Thoughts saved!"),
        Err(e) => error!("Error saving thoughts: {}", e),
    }
}

fn load_file_into_rows(file_path: &str) -> Result<Vec<Row>, csv::Error> {
    let mut rows: Vec<Row> = Vec::new();

    let reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(file_path);

    let mut reader = match reader {
        Ok(reader) => reader,
        Err(_) => return Ok(rows),
    };

    for result in reader.records() {
        let record: csv::StringRecord = result?;
        if record == csv::StringRecord::from(vec!["id", "timestamp", "message", "tags"]) {
            continue;
        }
        let row: Row = Row {
            id: record.get(0).unwrap().parse().unwrap(),
            timestamp: record.get(1).unwrap().parse().unwrap(),
            message: record.get(2).unwrap().to_string(),
            tags: match record.get(3) {
                Some(tags) => tags.to_string(),
                None => String::new(),
            },
        };
        rows.push(row);
    }

    Ok(rows)
}

fn save_rows_to_file(file_path: &str, rows: &Vec<Row>) -> Result<(), csv::Error> {
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_path(file_path)?;

    writer.write_record(&["id", "timestamp", "message", "tags"])?;

    for row in rows {
        writer.write_record(&[
            row.id.to_string(),
            row.timestamp.to_string(),
            row.message.to_string(),
            row.tags.to_string(),
        ])?;
    }

    writer.flush()?;

    Ok(())
}

fn remove_all_thoughts(mut rows: Vec<Row>) -> Vec<Row> {
    rows.clear();
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_file_into_rows() {
        let mut rows: Vec<Row> = Vec::new();
        // Test creating a new file
        let inexistent_file_path = String::from("tests/test_ingest_inexistent_file.csv");
        let result = load_file_into_rows(&inexistent_file_path);
        // Assert the two vectors are equal
        match result {
            Ok(result_rows) => assert_eq!(result_rows, rows),
            Err(_) => assert!(false),
        }

        // Test loading an existing file with some rows
        let file_path_with_data = String::from("tests/test_ingest_file_with_data.csv");
        let result = load_file_into_rows(&file_path_with_data);
        rows.push(Row {
            id: 1,
            timestamp: String::from("2018-01-01 00:00:00"),
            message: String::from("hello world"),
            tags: String::from("tag1"),
        });
        match result {
            Ok(result_rows) => assert_eq!(result_rows, rows),
            Err(_) => assert!(false),
        }
        rows.clear();

        // Test loading an existing file with no rows
        let empty_file_path = String::from("tests/test_ingest_empty_file.csv");
        let result = load_file_into_rows(&empty_file_path);

        // Assert the result is an empty vector
        match result {
            Ok(result_rows) => assert_eq!(result_rows, rows),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_save_rows_to_file() {
        let file_path = "tests/save_test.csv";
        let rows = vec![
            Row {
                id: 1,
                timestamp: "1627386000".to_string(),
                message: "Hello, world!".to_string(),
                tags: "test".to_string(),
            },
            Row {
                id: 2,
                timestamp: "1627386600".to_string(),
                message: "How are you?".to_string(),
                tags: "test".to_string(),
            },
        ];

        // Write the rows to a CSV file
        save_rows_to_file(file_path, &rows).unwrap();

        // Read the rows from the CSV file
        let loaded_rows = load_file_into_rows(file_path).unwrap();

        // Check that the loaded rows are equal to the original rows
        assert_eq!(loaded_rows, rows);

        // Clean up the test file
        std::fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_list_thoughts() {
        let rows = vec![
            Row {
                id: 1,
                timestamp: "2022-01-01".to_string(),
                message: "Test thought 1".to_string(),
                tags: "test1".to_string(),
            },
            Row {
                id: 2,
                timestamp: "2022-01-02".to_string(),
                message: "Test thought 2".to_string(),
                tags: "test2".to_string(),
            },
            Row {
                id: 3,
                timestamp: "2022-01-03".to_string(),
                message: "Test thought 3".to_string(),
                tags: "test1".to_string(),
            },
        ];

        // Test listing all thoughts
        let mut output = String::new();
        let expected_output = "ID, Timestamp, Thought, Tags\n\
                           1: 2022-01-01, Test thought 1, test1\n\
                           2: 2022-01-02, Test thought 2, test2\n\
                           3: 2022-01-03, Test thought 3, test1\n";
        list_thoughts(&rows, None);

        // Test listing thoughts with a specific tag
        let expected_output = "ID, Timestamp, Thought, Tags\n\
                           1: 2022-01-01, Test thought 1, test1\n\
                           3: 2022-01-03, Test thought 3, test1\n";
        list_thoughts(&rows, Some(&"test1".to_string()));
    }
}
