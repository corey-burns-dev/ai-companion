use serde::{Deserialize, Serialize};
use std::fs::{OpenOptions, File};
use std::io::{BufReader, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatEntry {
    pub role: String,
    pub content: String,
}

const HISTORY_PATH: &str = "chat_history.json";

pub fn append_to_history(entry: &ChatEntry) {
    let mut history = load_history();
    history.push(entry.clone());
    let json = serde_json::to_string_pretty(&history).unwrap();
    let mut file = File::create(HISTORY_PATH).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

pub fn load_history() -> Vec<ChatEntry> {
    if !Path::new(HISTORY_PATH).exists() {
        return Vec::new();
    }
    let file = File::open(HISTORY_PATH).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
}
