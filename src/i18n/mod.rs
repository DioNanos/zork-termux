pub mod loader;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "en")]
    English,
    #[serde(rename = "it")]
    Italian,
    #[serde(rename = "es")]
    Spanish,
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Italian => "it",
            Language::Spanish => "es",
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        let code = code.to_lowercase();
        let lang_code = code.split('_').next().unwrap_or(&code);
        match lang_code {
            "en" | "english" => Some(Language::English),
            "it" | "italian" | "italiano" => Some(Language::Italian),
            "es" | "spanish" | "español" => Some(Language::Spanish),
            _ => None,
        }
    }

    pub fn detect() -> Self {
        std::env::var("LANG")
            .ok()
            .and_then(|l| Language::from_code(&l))
            .unwrap_or(Language::English)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationData {
    pub meta: TranslationMeta,
    pub ui: UiStrings,
    pub help: HelpStrings,
    pub rooms: HashMap<String, RoomTranslation>,
    pub objects: HashMap<String, ObjectTranslation>,
    pub creatures: HashMap<String, CreatureTranslation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationMeta {
    pub language: String,
    pub version: String,
    pub author: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiStrings {
    pub welcome: String,
    pub goodbye: String,
    pub empty_handed: String,
    pub carrying: String,
    pub cant_go: String,
    pub unknown_command: String,
    pub taken: String,
    pub dropped: String,
    pub cant_take: String,
    pub cant_open: String,
    pub cant_close: String,
    pub already_open: String,
    pub already_closed: String,
    pub opened: String,
    pub closed: String,
    pub locked: String,
    pub unlocked: String,
    pub dont_see: String,
    pub not_holding: String,
    pub take_what: String,
    pub drop_what: String,
    pub examine_what: String,
    pub open_what: String,
    pub close_what: String,
    pub unlock_what: String,
    pub lock_what: String,
    pub contains: String,
    pub is_empty: String,
    pub score: String,
    pub moves: String,
    pub game_saved: String,
    pub game_restored: String,
    pub save_failed: String,
    pub restore_failed: String,
    pub no_saved_game: String,
    pub darkness: String,
    pub lamp_off: String,
    pub lamp_on: String,
    pub lamp_dim: String,
    pub lamp_out: String,
    pub attack_what: String,
    pub attack_with: String,
    pub killed: String,
    pub dodged: String,
    pub you_died: String,
    pub restart: String,
    pub inventory_full: String,
    pub cant_put_in: String,
    pub put_in: String,
    pub cant_read: String,
    pub cant_drink: String,
    pub drank: String,
    pub cant_eat: String,
    pub ate: String,
    pub already_have: String,
    pub cant_climb: String,
    pub cant_tie: String,
    pub tied: String,
    pub cant_untie: String,
    pub untied: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelpStrings {
    pub title: String,
    pub movement: String,
    pub look: String,
    pub inventory: String,
    pub take: String,
    pub drop: String,
    pub examine: String,
    pub open: String,
    pub close: String,
    pub put: String,
    pub read: String,
    pub use_: String,
    pub attack: String,
    pub score: String,
    pub save: String,
    pub restore: String,
    pub quit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomTranslation {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectTranslation {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub read: Option<String>,
    #[serde(default)]
    pub on_desc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatureTranslation {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub dead_desc: Option<String>,
    #[serde(default)]
    pub attacks: Option<String>,
    #[serde(default)]
    pub sleeping: Option<String>,
    #[serde(default)]
    pub steals: Option<String>,
}

pub struct I18n {
    language: Language,
    data: TranslationData,
}

impl I18n {
    pub fn load(lang: Language) -> Result<Self, String> {
        let data = loader::load_translation(lang)?;
        Ok(I18n {
            language: lang,
            data,
        })
    }

    pub fn language(&self) -> Language {
        self.language
    }

    pub fn ui(&self) -> &UiStrings {
        &self.data.ui
    }

    pub fn help(&self) -> &HelpStrings {
        &self.data.help
    }

    pub fn room(&self, id: &str) -> Option<&RoomTranslation> {
        self.data.rooms.get(id)
    }

    pub fn object(&self, id: &str) -> Option<&ObjectTranslation> {
        self.data.objects.get(id)
    }

    pub fn creature(&self, id: &str) -> Option<&CreatureTranslation> {
        self.data.creatures.get(id)
    }

    pub fn format(&self, template: &str, vars: &[(&str, &str)]) -> String {
        let mut result = template.to_string();
        for (key, value) in vars {
            result = result.replace(&format!("{{{}}}", key), value);
        }
        result
    }

    pub fn help_text(&self) -> String {
        let h = &self.data.help;
        format!(
            "╔══════════════════════════════╗\n\
             ║ {:^28} ║\n\
             ╠══════════════════════════════╣\n\
             ║ {} ║\n\
             ║ {} ║\n\
             ║ {} ║\n\
             ║ {} ║\n\
             ║ {} ║\n\
             ║ {} ║\n\
             ║ {} ║\n\
             ║ {} ║\n\
             ║ {} ║\n\
             ║ {} ║\n\
             ║ {} ║\n\
             ║ {} ║\n\
             ║ {} ║\n\
             ║ {} ║\n\
             ║ {} ║\n\
             ╚══════════════════════════════╝",
            h.title,
            h.movement,
            h.look,
            h.inventory,
            h.take,
            h.drop,
            h.examine,
            h.open,
            h.close,
            h.put,
            h.read,
            h.attack,
            h.score,
            h.save,
            h.restore,
            h.quit
        )
    }

    pub fn score_text(&self, score: u32, moves: u32) -> String {
        format!(
            "{}: {}\n{}: {}",
            self.data.ui.score, score, self.data.ui.moves, moves
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{I18n, Language};

    #[test]
    fn advertisement_is_readable_in_all_languages() {
        for lang in [Language::English, Language::Italian, Language::Spanish] {
            let i18n = I18n::load(lang).expect("translation should load");
            let obj = i18n
                .object("advertisement")
                .expect("advertisement translation should exist");
            assert!(
                obj.read.is_some(),
                "advertisement should define read text for {}",
                lang.code()
            );
        }
    }
}
