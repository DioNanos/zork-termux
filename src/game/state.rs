use crate::i18n::Language;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub current_room: String,
    pub inventory: Vec<String>,
    pub language: String,
    pub score: u32,
    pub moves: u32,
    pub visited_rooms: Vec<String>,
    pub lamp_lit: bool,
    pub lamp_turns: u32,
    pub player_dead: bool,
    pub object_states: std::collections::HashMap<String, ObjectState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectState {
    pub location: String,
    pub is_open: bool,
    pub is_lit: bool,
}

impl GameState {
    pub fn new(lang: Language, start_room: &str) -> Self {
        GameState {
            current_room: start_room.to_string(),
            inventory: Vec::new(),
            language: lang.code().to_string(),
            score: 0,
            moves: 0,
            visited_rooms: vec![start_room.to_string()],
            lamp_lit: false,
            lamp_turns: 0,
            player_dead: false,
            object_states: std::collections::HashMap::new(),
        }
    }

    pub fn language(&self) -> Language {
        Language::from_code(&self.language).unwrap_or(Language::English)
    }

    pub fn move_to(&mut self, room_id: &str) {
        self.current_room = room_id.to_string();
        if !self.visited_rooms.contains(&self.current_room) {
            self.visited_rooms.push(self.current_room.clone());
        }
    }

    pub fn add_to_inventory(&mut self, item: String) {
        if !self.inventory.contains(&item) {
            self.inventory.push(item);
        }
    }

    pub fn remove_from_inventory(&mut self, item: &str) -> Option<String> {
        if let Some(pos) = self.inventory.iter().position(|x| x == item) {
            Some(self.inventory.remove(pos))
        } else {
            None
        }
    }

    pub fn has_item(&self, item: &str) -> bool {
        self.inventory
            .iter()
            .any(|i| i.to_lowercase() == item.to_lowercase())
    }

    pub fn add_score(&mut self, points: u32) {
        self.score += points;
    }

    pub fn save(&self, slot: u32) -> Result<(), String> {
        let save_path = Self::save_path(slot);

        let content = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize: {}", e))?;

        if let Some(parent) = save_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create save directory: {}", e))?;
        }

        fs::write(&save_path, content).map_err(|e| format!("Failed to write save: {}", e))?;

        Ok(())
    }

    pub fn load(slot: u32) -> Result<Self, String> {
        let save_path = Self::save_path(slot);

        if !save_path.exists() {
            return Err("No saved game found".to_string());
        }

        let content =
            fs::read_to_string(&save_path).map_err(|e| format!("Failed to read save: {}", e))?;

        serde_json::from_str(&content).map_err(|e| format!("Failed to parse save: {}", e))
    }

    pub fn save_path(slot: u32) -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home)
            .join(".zork-termux")
            .join(format!("save_{}.json", slot))
    }

    pub fn save_exists(slot: u32) -> bool {
        Self::save_path(slot).exists()
    }
}
