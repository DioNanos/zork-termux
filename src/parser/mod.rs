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
        let object = if tokens.len() > 1 {
            Some(tokens[1..].join(" "))
        } else {
            None
        };

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
            "r" | "read" => Verb::Read,
            "use" => Verb::Use,
            "attack" | "kill" | "hit" | "fight" => Verb::Attack,
            "put" | "insert" | "place" => Verb::Put,
            "save" => Verb::Save,
            "restore" | "load" => Verb::Restore,
            "help" | "?" => Verb::Help,
            "score" => Verb::Score,
            "enter" | "in" | "go" => Verb::Enter,
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
            "giu" | "giù" | "basso" => Verb::Down,
            "l" | "guarda" | "look" => Verb::Look,
            "i" | "inv" | "inventario" => Verb::Inventory,
            "prendi" | "take" | "raccogli" => Verb::Take,
            "posa" | "drop" | "lascia" => Verb::Drop,
            "x" | "esamina" | "ex" => Verb::Examine,
            "apri" | "open" => Verb::Open,
            "chiudi" | "close" => Verb::Close,
            "r" | "leggi" | "read" => Verb::Read,
            "usa" | "use" => Verb::Use,
            "attacca" | "uccidi" | "colpisci" | "attack" | "kill" => Verb::Attack,
            "metti" | "inserisci" | "put" => Verb::Put,
            "salva" | "save" => Verb::Save,
            "ripristina" | "carica" | "restore" | "load" => Verb::Restore,
            "aiuto" | "?" | "help" => Verb::Help,
            "punti" | "score" => Verb::Score,
            "entra" | "in" | "vai" => Verb::Enter,
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
            "r" | "leer" | "read" => Verb::Read,
            "usar" | "use" => Verb::Use,
            "atacar" | "matar" | "golpear" | "attack" | "kill" => Verb::Attack,
            "poner" | "mete" | "insertar" | "put" => Verb::Put,
            "guardar" | "save" => Verb::Save,
            "restaurar" | "cargar" | "restore" | "load" => Verb::Restore,
            "ayuda" | "?" | "help" => Verb::Help,
            "puntos" | "score" => Verb::Score,
            "entrar" | "entra" | "en" | "ir" => Verb::Enter,
            _ => Verb::Unknown(word.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_missing_english_verbs() {
        let parser = Parser::new(Language::English);
        assert!(matches!(parser.parse("save").unwrap().verb, Verb::Save));
        assert!(matches!(
            parser.parse("restore").unwrap().verb,
            Verb::Restore
        ));
        assert!(matches!(
            parser.parse("read note").unwrap().verb,
            Verb::Read
        ));
        assert!(matches!(parser.parse("use lamp").unwrap().verb, Verb::Use));
        assert!(matches!(
            parser.parse("attack troll").unwrap().verb,
            Verb::Attack
        ));
        assert!(matches!(
            parser.parse("put coin in mailbox").unwrap().verb,
            Verb::Put
        ));
    }

    #[test]
    fn parses_missing_italian_verbs() {
        let parser = Parser::new(Language::Italian);
        assert!(matches!(parser.parse("salva").unwrap().verb, Verb::Save));
        assert!(matches!(
            parser.parse("ripristina").unwrap().verb,
            Verb::Restore
        ));
        assert!(matches!(
            parser.parse("leggi volantino").unwrap().verb,
            Verb::Read
        ));
        assert!(matches!(
            parser.parse("usa lanterna").unwrap().verb,
            Verb::Use
        ));
        assert!(matches!(
            parser.parse("attacca troll").unwrap().verb,
            Verb::Attack
        ));
        assert!(matches!(
            parser.parse("metti moneta in cassetta").unwrap().verb,
            Verb::Put
        ));
    }

    #[test]
    fn parses_missing_spanish_verbs() {
        let parser = Parser::new(Language::Spanish);
        assert!(matches!(parser.parse("guardar").unwrap().verb, Verb::Save));
        assert!(matches!(
            parser.parse("restaurar").unwrap().verb,
            Verb::Restore
        ));
        assert!(matches!(
            parser.parse("leer folleto").unwrap().verb,
            Verb::Read
        ));
        assert!(matches!(
            parser.parse("usar linterna").unwrap().verb,
            Verb::Use
        ));
        assert!(matches!(
            parser.parse("atacar troll").unwrap().verb,
            Verb::Attack
        ));
        assert!(matches!(
            parser.parse("poner moneda en buzon").unwrap().verb,
            Verb::Put
        ));
    }

    #[test]
    fn parses_multiword_object() {
        let parser = Parser::new(Language::English);
        let cmd = parser.parse("take rusty brass key").unwrap();
        assert_eq!(cmd.object.as_deref(), Some("rusty brass key"));
    }
}
