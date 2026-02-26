use std::io::{self, Write};
use zork_termux::game::{Game, GameChoice};
use zork_termux::i18n::{I18n, Language};
use zork_termux::logging;

const MOBILE_DEFAULT_WIDTH: usize = 40;
const MIN_BOX_WIDTH: usize = 30;
const MAX_BOX_WIDTH: usize = 56;

fn main() {
    let session_log = logging::init();

    let width = detect_box_width();
    print_banner(width);
    print_log_hint(session_log.as_deref());

    let language = select_language(width);
    let game = select_game(&language, width);
    logging::info(format!(
        "menu.selection language={} game={:?}",
        language.code(),
        game
    ));

    match I18n::load(language) {
        Ok(i18n) => {
            let mut game = Game::new(i18n, game);
            game.run();
        }
        Err(e) => {
            logging::error(format!("i18n.load.failed error={}", e));
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

    let usable = columns.max(24);
    let lower = MIN_BOX_WIDTH.min(usable);
    usable.clamp(lower, MAX_BOX_WIDTH)
}

fn print_banner(width: usize) {
    println!();
    print_box(
        width,
        "ZORK-TERMUX v0.5",
        &[
            "Terminal Adventure Engine".to_string(),
            "Mobile-first UI · Linux/macOS ready".to_string(),
        ],
    );
    println!();
}

fn print_log_hint(log_path: Option<&std::path::Path>) {
    if let Some(path) = log_path {
        println!("Log file: {}", path.display());
        println!();
    }
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
        for wrapped in wrap_text(line, inner) {
            print_left(&wrapped, inner);
        }
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
    println!("║{}{}{}║", " ".repeat(left_pad), fit, " ".repeat(right_pad));
}

fn print_left(text: &str, inner: usize) {
    let fit = text.chars().take(inner).collect::<String>();
    let pad = inner.saturating_sub(fit.chars().count());
    println!("║{}{}║", fit, " ".repeat(pad));
}

fn wrap_text(text: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![String::new()];
    }

    if text.trim().is_empty() {
        return vec![String::new()];
    }

    let mut lines = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        let word_len = word.chars().count();

        if word_len > width {
            if !current.is_empty() {
                lines.push(current);
                current = String::new();
            }

            let mut chunk = String::new();
            for ch in word.chars() {
                chunk.push(ch);
                if chunk.chars().count() == width {
                    lines.push(chunk);
                    chunk = String::new();
                }
            }

            if !chunk.is_empty() {
                current = chunk;
            }

            continue;
        }

        if current.is_empty() {
            current.push_str(word);
            continue;
        }

        let candidate_len = current.chars().count() + 1 + word_len;
        if candidate_len <= width {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(current);
            current = word.to_string();
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
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
