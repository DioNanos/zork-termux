pub mod command;

use crate::i18n::Language;
pub use command::{Command, Verb};

pub struct Parser {
    language: Language,
}

impl Parser {
    pub fn new(language: Language) -> Self {
        Parser { language }
    }

    pub fn parse(&self, input: &str) -> Option<Command> {
        let tokens: Vec<&str> = input.split_whitespace().collect();

        if tokens.is_empty() {
            return None;
        }

        let verb = self.parse_verb(tokens[0]);
        let object = tokens.get(1).map(|s| s.to_string());

        Some(Command { verb, object })
    }

    fn parse_verb(&self, word: &str) -> Verb {
        let word = word.to_lowercase();

        match self.language {
            Language::English => self.parse_verb_en(&word),
            Language::Italian => self.parse_verb_it(&word),
            Language::Spanish => self.parse_verb_es(&word),
        }
    }

    fn parse_verb_en(&self, word: &str) -> Verb {
        match word {
            "n" | "north" => Verb::North,
            "s" | "south" => Verb::South,
            "e" | "east" => Verb::East,
            "w" | "west" => Verb::West,
            "u" | "up" => Verb::Up,
            "d" | "down" => Verb::Down,
            "l" | "look" => Verb::Look,
            "i" | "inv" | "inventory" => Verb::Inventory,
            "take" | "get" | "pick" => Verb::Take,
            "drop" => Verb::Drop,
            "x" | "examine" | "ex" => Verb::Examine,
            "open" => Verb::Open,
            "close" => Verb::Close,
            "help" | "?" => Verb::Help,
            "score" => Verb::Score,
            _ => Verb::Unknown(word.to_string()),
        }
    }

    fn parse_verb_it(&self, word: &str) -> Verb {
        match word {
            "n" | "nord" => Verb::North,
            "s" | "sud" => Verb::South,
            "e" | "est" => Verb::East,
            "o" | "ovest" => Verb::West,
            "su" | "alto" => Verb::Up,
            "giù" | "basso" => Verb::Down,
            "l" | "guarda" | "look" => Verb::Look,
            "i" | "inv" | "inventario" => Verb::Inventory,
            "prendi" | "take" | "raccogli" => Verb::Take,
            "posa" | "drop" | "lascia" => Verb::Drop,
            "x" | "esamina" | "ex" => Verb::Examine,
            "apri" | "open" => Verb::Open,
            "chiudi" | "close" => Verb::Close,
            "aiuto" | "?" | "help" => Verb::Help,
            "punti" | "score" => Verb::Score,
            _ => Verb::Unknown(word.to_string()),
        }
    }

    fn parse_verb_es(&self, word: &str) -> Verb {
        match word {
            "n" | "norte" => Verb::North,
            "s" | "sur" => Verb::South,
            "e" | "este" => Verb::East,
            "o" | "oeste" => Verb::West,
            "arriba" | "subir" => Verb::Up,
            "abajo" | "bajar" => Verb::Down,
            "l" | "mirar" | "look" => Verb::Look,
            "i" | "inv" | "inventario" => Verb::Inventory,
            "tomar" | "coger" | "take" => Verb::Take,
            "soltar" | "drop" => Verb::Drop,
            "x" | "examinar" | "ex" => Verb::Examine,
            "abrir" | "open" => Verb::Open,
            "cerrar" | "close" => Verb::Close,
            "ayuda" | "?" | "help" => Verb::Help,
            "puntos" | "score" => Verb::Score,
            _ => Verb::Unknown(word.to_string()),
        }
    }
}
