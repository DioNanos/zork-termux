pub mod actions;
pub mod state;
pub mod world;

use crate::i18n::I18n;
use crate::i18n::Language;
use crate::parser::Parser;
use state::GameState;
use world::World;

#[derive(Debug, Clone, Copy)]
pub enum GameChoice {
    Zork1,
    Zork2,
    Zork3,
}

pub struct Game {
    state: GameState,
    world: World,
    parser: Parser,
    i18n: I18n,
    choice: GameChoice,
}

impl Game {
    pub fn new(i18n: I18n, choice: GameChoice) -> Self {
        let lang = i18n.language();
        let (world, start_room) = match choice {
            GameChoice::Zork1 => (World::load_zork1(), "west_of_house"),
            GameChoice::Zork2 => (World::load_zork2(), "inside_barrow"),
            GameChoice::Zork3 => (World::load_zork3(), "cp_ante"),
        };

        Game {
            state: GameState::new(lang, start_room),
            world,
            parser: Parser::new(lang),
            i18n,
            choice,
        }
    }

    pub fn run(&mut self) {
        self.show_intro();
        self.show_room();

        loop {
            let input = self.read_input();

            if input.is_empty() {
                continue;
            }

            if self.is_quit(&input) {
                self.show_goodbye();
                break;
            }

            if let Some(cmd) = self.parser.parse(&input) {
                self.execute(cmd);
            }
        }
    }

    fn show_intro(&self) {
        let title = match self.choice {
            GameChoice::Zork1 => "ZORK I: THE GREAT UNDERGROUND EMPIRE",
            GameChoice::Zork2 => "ZORK II: THE WIZARD OF FROBOZZ",
            GameChoice::Zork3 => "ZORK III: THE DUNGEON MASTER",
        };
        println!("{}", self.i18n.ui().welcome);
        println!("\n{}\n", title);
    }

    fn show_room(&self) {
        let room_id = &self.state.current_room;

        if let Some(room_trans) = self.i18n.room(room_id) {
            println!("\n{}\n", room_trans.name);
            println!("{}\n", room_trans.description);
        } else {
            let room = self.world.get_room(room_id);
            println!("\n{}\n", room.name);
        }
    }

    fn read_input(&self) -> String {
        use std::io::{self, Write};
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_lowercase()
    }

    fn is_quit(&self, input: &str) -> bool {
        match self.i18n.language() {
            Language::English => matches!(input, "quit" | "exit" | "q"),
            Language::Italian => matches!(input, "quit" | "exit" | "q" | "esci" | "fine"),
            Language::Spanish => matches!(input, "quit" | "exit" | "q" | "salir" | "fin"),
        }
    }

    fn show_goodbye(&self) {
        println!("\n{}", self.i18n.ui().goodbye);
    }

    fn execute(&mut self, cmd: crate::parser::Command) {
        actions::execute(&mut self.state, &mut self.world, cmd, &self.i18n);
    }
}
