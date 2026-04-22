use std::collections::HashMap;
use crate::models::file_record::FileRecord;

pub struct DiffResult {
    pub added: Vec<FileRecord>,
    pub modified: Vec<FileRecord>,
    pub deleted: Vec<FileRecord>,
}

pub fn compute_diff(
    old: &HashMap<String, FileRecord>,
    new: &HashMap<String, FileRecord>,
) -> DiffResult {

    let mut added = Vec::new();
    let mut modified = Vec::new();
    let mut deleted = Vec::new();

    // Detect added & modified
    for (path, new_file) in new {
        match old.get(path) {
            None => added.push(new_file.clone()),
            Some(old_file) => {
                if old_file.hash != new_file.hash {
                    modified.push(new_file.clone());
                }
            }
        }
    }

    // Detect deleted
    for (path, old_file) in old {
        if !new.contains_key(path) {
            deleted.push(old_file.clone());
        }
    }

    DiffResult {
        added,
        modified,
        deleted,
    }
}