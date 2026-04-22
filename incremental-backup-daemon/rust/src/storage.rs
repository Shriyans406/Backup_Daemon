use std::fs;
use crate::models::snapshot::Snapshot;
use serde_json::Value;

pub fn load_snapshot(path: &str) -> Option<Snapshot> {
    let data = fs::read_to_string(path).ok()?;
    serde_json::from_str(&data).ok()
}

pub fn save_snapshot(path: &str, snapshot: &Snapshot) {
    let json = serde_json::to_string_pretty(snapshot).unwrap();
    fs::write(path, json).expect("Failed to write snapshot");
}

pub fn update_index(timestamp: &str) {
    let index_path = "../storage/index.json";

    let mut data: Vec<Value> = if let Ok(content) = fs::read_to_string(index_path) {
        serde_json::from_str(&content).unwrap_or(Vec::new())
    } else {
        Vec::new()
    };

    let entry = serde_json::json!({
        "timestamp": timestamp,
        "snapshot": format!("snapshots/{}.json", timestamp),
        "backup_dir": format!("backups/{}", timestamp)
    });

    data.push(entry);

    let updated = serde_json::to_string_pretty(&data).unwrap();
    fs::write(index_path, updated).expect("Failed to update index");
}