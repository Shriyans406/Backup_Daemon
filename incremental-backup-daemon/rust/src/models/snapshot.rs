use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use super::file_record::FileRecord;

#[derive(Debug, Serialize, Deserialize)]
pub struct Snapshot {
    pub timestamp: String,
    pub files: HashMap<String, FileRecord>,
}