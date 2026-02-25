use std::io::{self, Write};
use zork_termux::game::{Game, GameChoice};
use zork_termux::i18n::{I18n, Language};

const MOBILE_DEFAULT_WIDTH: usize = 36;
const MIN_BOX_WIDTH: usize = 30;
const MAX_BOX_WIDTH: usize = 48;

fn main() {
    let width = detect_box_width();
    print_banner(width);

    let language = select_language(width);
    let game = select_game(&language, width);

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

fn detect_box_width() -> usize {
    let columns = std::env::var("COLUMNS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(MOBILE_DEFAULT_WIDTH);

    columns
        .saturating_sub(2)
        .clamp(MIN_BOX_WIDTH, MAX_BOX_WIDTH)
}

fn print_banner(width: usize) {
    println!();
    print_box(
        width,
        "ZORK-TERMUX v0.4",
        &[
            "Terminal Adventure Engine".to_string(),
            "Mobile-first UI · Linux/macOS ready".to_string(),
        ],
    );
    println!();
}

fn select_language(width: usize) -> Language {
    let detected = Language::detect();

    print_box(
        width,
        "LANGUAGE / LINGUA / IDIOMA",
        &[
            "[1] English".to_string(),
            "[2] Italiano".to_string(),
            "[3] Espanol".to_string(),
            format!("[Enter] Auto ({})", detected.code().to_uppercase()),
        ],
    );

    let input = read_prompt();

    match input.trim() {
        "1" | "en" | "english" => Language::English,
        "2" | "it" | "italiano" => Language::Italian,
        "3" | "es" | "español" => Language::Spanish,
        "" => detected,
        _ => detected, // fallback keeps mobile menu fast and forgiving
    }
}

fn select_game(lang: &Language, width: usize) -> GameChoice {
    let (title, options) = match lang {
        Language::English => (
            "SELECT GAME",
            vec![
                "[1] Zork I  - Great Underground Empire",
                "[2] Zork II - Wizard of Frobozz",
                "[3] Zork III - The Dungeon Master",
            ],
        ),
        Language::Italian => (
            "SCEGLI GIOCO",
            vec![
                "[1] Zork I  - Grande Impero Sotterraneo",
                "[2] Zork II - Il Mago di Frobozz",
                "[3] Zork III - Il Dungeon Master",
            ],
        ),
        Language::Spanish => (
            "ELIGE JUEGO",
            vec![
                "[1] Zork I  - Gran Imperio Subterraneo",
                "[2] Zork II - El Mago de Frobozz",
                "[3] Zork III - El Dungeon Master",
            ],
        ),
    };

    println!();
    let option_lines = options.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    print_box(width, title, &option_lines);

    let input = read_prompt();

    match input.trim() {
        "1" | "zork1" | "i" => GameChoice::Zork1,
        "2" | "zork2" | "ii" => GameChoice::Zork2,
        "3" | "zork3" | "iii" => GameChoice::Zork3,
        _ => GameChoice::Zork1,
    }
}

fn read_prompt() -> String {
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_lowercase()
}

fn print_box(width: usize, title: &str, lines: &[String]) {
    let inner = width.saturating_sub(2);
    print_border('╔', '═', '╗', inner);
    print_centered(title, inner);
    print_border('╠', '═', '╣', inner);
    for line in lines {
        print_left(line, inner);
    }
    print_border('╚', '═', '╝', inner);
}

fn print_border(left: char, fill: char, right: char, inner: usize) {
    println!("{}{}{}", left, fill.to_string().repeat(inner), right);
}

fn print_centered(text: &str, inner: usize) {
    let fit = fit_text(text, inner);
    let len = fit.chars().count();
    let left_pad = inner.saturating_sub(len) / 2;
    let right_pad = inner.saturating_sub(len + left_pad);
    println!(
        "║{}{}{}║",
        " ".repeat(left_pad),
        fit,
        " ".repeat(right_pad)
    );
}

fn print_left(text: &str, inner: usize) {
    let fit = fit_text(text, inner);
    let pad = inner.saturating_sub(fit.chars().count());
    println!("║{}{}║", fit, " ".repeat(pad));
}

fn fit_text(text: &str, width: usize) -> String {
    let chars = text.chars().collect::<Vec<_>>();
    if chars.len() <= width {
        return text.to_string();
    }

    if width <= 3 {
        return ".".repeat(width);
    }

    let mut shortened = chars[..width - 3].iter().collect::<String>();
    shortened.push_str("...");
    shortened
}
