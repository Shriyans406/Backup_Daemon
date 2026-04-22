mod parser;
mod diff;
mod storage;
mod models {
    pub mod file_record;
    pub mod snapshot;
}

use parser::parse_current_state;
use diff::compute_diff;
use storage::{load_snapshot, save_snapshot, update_index};
use models::snapshot::Snapshot;

use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::{self, File};
use std::io::Write;

fn main() {
    let current_file = "../storage/temp/current_state.txt";
    let latest_snapshot_path = "../storage/snapshots/latest.json";

    println!("[RUST] Loading current state...");
    let current_state = parse_current_state(current_file);

    println!("[RUST] Loading previous snapshot...");
    let old_snapshot = load_snapshot(latest_snapshot_path);

    let old_state = match old_snapshot {
        Some(snapshot) => snapshot.files,
        None => {
            println!("[RUST] No previous snapshot found.");
            std::collections::HashMap::new()
        }
    };

    println!("[RUST] Computing diff...");
    let diff = compute_diff(&old_state, &current_state);

    println!("--- DIFF RESULT ---");
    println!("Added: {}", diff.added.len());
    println!("Modified: {}", diff.modified.len());
    println!("Deleted: {}", diff.deleted.len());

    // Generate timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();

    // Write timestamp for Bash
    let mut ts_file = File::create("../storage/temp/timestamp.txt")
        .expect("Unable to create timestamp file");
    writeln!(ts_file, "{}", timestamp).unwrap();

    // Write changed files
    println!("[RUST] Writing changed files...");
    let mut changed_file = File::create("../storage/temp/changed_files.txt")
        .expect("Unable to create changed files list");

    for f in diff.added.iter().chain(diff.modified.iter()) {
        writeln!(changed_file, "{}", f.path).unwrap();
    }

    // Save snapshot with timestamp
    let snapshot_path = format!("../storage/snapshots/{}.json", timestamp);

    let new_snapshot = Snapshot {
        timestamp: timestamp.clone(),
        files: current_state,
    };

    println!("[RUST] Saving snapshot...");
    save_snapshot(&snapshot_path, &new_snapshot);

    // Update latest snapshot pointer
    fs::copy(&snapshot_path, latest_snapshot_path)
        .expect("Failed to update latest snapshot");

    // Update index
    update_index(&timestamp);

    println!("[RUST] Done.");
}