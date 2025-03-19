//! config.rs
//! Handles game configuration persistence.

use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub player_name: String,
    pub last_song: usize,
    pub high_score: u32,
    pub line_count: u32,
    pub game_mode: String,
}

pub fn load_config() -> Config {
    if let Ok(data) = fs::read_to_string("config.json") {
        if let Ok(config) = serde_json::from_str(&data) {
            return config;
        }
    }
    Config {
        player_name: "Player".to_string(),
        last_song: 0,
        high_score: 0,
        line_count: 0,
        game_mode: "Classic".to_string(),
    }
}

pub fn save_config(config: &Config) {
    if let Ok(json) = serde_json::to_string_pretty(config) {
        let _ = fs::write("config.json", json);
    }
}
