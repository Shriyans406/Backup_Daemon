mod parser;
mod diff;
mod storage;
mod models {
    pub mod file_record;
    pub mod snapshot;
}

use parser::parse_current_state;
use diff::compute_diff;
use storage::{load_snapshot, save_snapshot};
use models::snapshot::Snapshot;
use std::time::{SystemTime, UNIX_EPOCH};

use std::fs::File;
use std::io::Write;

fn main() {
    let current_file = "../storage/temp/current_state.txt";
    let snapshot_file = "../storage/snapshots/latest.json";

    println!("[RUST] Loading current state...");
    let current_state = parse_current_state(current_file);

    println!("[RUST] Loading previous snapshot...");
    let old_snapshot = load_snapshot(snapshot_file);

    let old_state = match old_snapshot {
        Some(snapshot) => snapshot.files,
        None => {
            println!("[RUST] No previous snapshot found.");
            std::collections::HashMap::new()
        }
    };

    println!("[RUST] Computing diff...");
    let diff = compute_diff(&old_state, &current_state);

    println!("[RUST] Writing changed files...");

let mut file = File::create("../storage/temp/changed_files.txt")
    .expect("Unable to create file");

// Write added + modified files
for f in diff.added.iter().chain(diff.modified.iter()) {
    writeln!(file, "{}", f.path).unwrap();
}


    println!("--- DIFF RESULT ---");
    println!("Added: {}", diff.added.len());
    println!("Modified: {}", diff.modified.len());
    println!("Deleted: {}", diff.deleted.len());

    // Create new snapshot
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();

    let new_snapshot = Snapshot {
        timestamp,
        files: current_state,
    };

    println!("[RUST] Saving snapshot...");
    save_snapshot(snapshot_file, &new_snapshot);

    println!("[RUST] Done.");
}