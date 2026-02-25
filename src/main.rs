use std::io::{self, Write};
use zork_termux::game::{Game, GameChoice};
use zork_termux::i18n::{I18n, Language};

fn main() {
    println!("\n╔════════════════════════════════╗");
    println!("║        ZORK-TERMUX v0.3        ║");
    println!("║      Multi-language Zork       ║");
    println!("╚════════════════════════════════╝\n");

    let language = select_language();
    let game = select_game(&language);

    match I18n::load(language) {
        Ok(i18n) => {
            let mut game = Game::new(i18n, game);
            game.run();
        }
        Err(e) => {
            eprintln!("Error loading translations: {}", e);
            std::process::exit(1);
        }
    }
}

fn select_language() -> Language {
    let detected = Language::detect();

    println!("╔════════════════════════════════╗");
    println!("║        SELECT LANGUAGE         ║");
    println!("║      SCEGLI LINGUA             ║");
    println!("║      ELIGE IDIOMA              ║");
    println!("╠════════════════════════════════╣");
    println!("║  [1] English                   ║");
    println!("║  [2] Italiano                  ║");
    println!("║  [3] Español                   ║");
    println!("╠════════════════════════════════╣");
    println!(
        "║  [Enter] Auto-detect ({})      ║",
        detected.code().to_uppercase()
    );
    println!("╚════════════════════════════════╝");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "1" | "en" | "english" => Language::English,
        "2" | "it" | "italiano" => Language::Italian,
        "3" | "es" | "español" => Language::Spanish,
        "" => detected,
        _ => detected,
    }
}

fn select_game(lang: &Language) -> GameChoice {
    let (title, options) = match lang {
        Language::English => (
            "SELECT GAME",
            vec![
                "[1] Zork I - The Great Underground Empire",
                "[2] Zork II - The Wizard of Frobozz",
                "[3] Zork III - The Dungeon Master",
            ],
        ),
        Language::Italian => (
            "SCEGLI GIOCO",
            vec![
                "[1] Zork I - Il Grande Impero Sotterraneo",
                "[2] Zork II - Il Mago di Frobozz",
                "[3] Zork III - Il Dungeon Master",
            ],
        ),
        Language::Spanish => (
            "ELIGE JUEGO",
            vec![
                "[1] Zork I - El Gran Imperio Subterráneo",
                "[2] Zork II - El Mago de Frobozz",
                "[3] Zork III - El Dungeon Master",
            ],
        ),
    };

    println!("\n╔════════════════════════════════╗");
    println!("║{:^32}║", title);
    println!("╠════════════════════════════════╣");
    for opt in &options {
        println!("║  {:<30}║", opt);
    }
    println!("╚════════════════════════════════╝");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "1" | "zork1" | "i" => GameChoice::Zork1,
        "2" | "zork2" | "ii" => GameChoice::Zork2,
        "3" | "zork3" | "iii" => GameChoice::Zork3,
        _ => GameChoice::Zork1,
    }
}
