use std::fs;
use crate::models::snapshot::Snapshot;

pub fn load_snapshot(path: &str) -> Option<Snapshot> {
    let data = fs::read_to_string(path).ok()?;
    serde_json::from_str(&data).ok()
}

pub fn save_snapshot(path: &str, snapshot: &Snapshot) {
    let json = serde_json::to_string_pretty(snapshot).unwrap();
    fs::write(path, json).expect("Failed to write snapshot");
}