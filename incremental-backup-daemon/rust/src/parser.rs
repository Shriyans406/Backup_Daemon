use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::models::file_record::FileRecord;
use std::collections::HashMap;

pub fn parse_current_state(file_path: &str) -> HashMap<String, FileRecord> {
    let file = File::open(file_path).expect("Failed to open state file");
    let reader = BufReader::new(file);

    let mut files_map = HashMap::new();

    for line in reader.lines() {
        if let Ok(record) = line {
            let parts: Vec<&str> = record.split('|').collect();

            if parts.len() != 4 {
                continue;
            }

            let file_record = FileRecord {
                path: parts[0].to_string(),
                size: parts[1].parse().unwrap_or(0),
                mtime: parts[2].parse().unwrap_or(0),
                hash: parts[3].to_string(),
            };

            files_map.insert(file_record.path.clone(), file_record);
        }
    }

    files_map
}