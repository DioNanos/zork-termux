use super::{Language, TranslationData};
use std::fs;
use std::path::Path;

const EMBEDDED_EN: &str = include_str!("../../data/i18n/en.json");
const EMBEDDED_IT: &str = include_str!("../../data/i18n/it.json");
const EMBEDDED_ES: &str = include_str!("../../data/i18n/es.json");

pub fn load_translation(lang: Language) -> Result<TranslationData, String> {
    let json_str = match lang {
        Language::English => EMBEDDED_EN,
        Language::Italian => EMBEDDED_IT,
        Language::Spanish => EMBEDDED_ES,
    };

    serde_json::from_str(json_str).map_err(|e| format!("Failed to parse translation: {}", e))
}

pub fn load_translation_from_file(path: &Path) -> Result<TranslationData, String> {
    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read translation file: {}", e))?;

    serde_json::from_str(&content).map_err(|e| format!("Failed to parse translation: {}", e))
}

pub fn save_translation(data: &TranslationData, path: &Path) -> Result<(), String> {
    let content = serde_json::to_string_pretty(data)
        .map_err(|e| format!("Failed to serialize translation: {}", e))?;

    fs::write(path, content).map_err(|e| format!("Failed to write translation file: {}", e))
}
