use crate::game::state::GameState;
use crate::game::world::World;
use crate::i18n::I18n;
use crate::parser::Command;
use crate::parser::Verb;

pub fn execute(state: &mut GameState, world: &mut World, cmd: Command, i18n: &I18n) {
    state.moves += 1;
    let ui = i18n.ui();

    match cmd.verb {
        Verb::Look => cmd_look(state, world, i18n),
        Verb::Inventory => cmd_inventory(state, ui),
        Verb::North | Verb::South | Verb::East | Verb::West | Verb::Up | Verb::Down => {
            cmd_move(state, world, &cmd.verb, i18n);
        }
        Verb::Take => cmd_take(state, world, cmd.object.as_deref(), i18n),
        Verb::Drop => cmd_drop(state, world, cmd.object.as_deref(), i18n),
        Verb::Examine => cmd_examine(state, world, cmd.object.as_deref(), i18n),
        Verb::Open => cmd_open(state, world, cmd.object.as_deref(), i18n),
        Verb::Close => cmd_close(state, world, cmd.object.as_deref(), i18n),
        Verb::Read => cmd_read(state, world, cmd.object.as_deref(), i18n),
        Verb::Use => cmd_use(state, cmd.object.as_deref(), i18n),
        Verb::Help => println!("\n{}", i18n.help_text()),
        Verb::Score => println!("\n{}", i18n.score_text(state.score, state.moves)),
        Verb::Save => match state.save(1) {
            Ok(()) => println!("\n{}", ui.game_saved),
            Err(e) => println!("\n{}: {}", ui.save_failed, e),
        },
        Verb::Restore => {
            println!("\n{}", ui.game_restored);
        }
        Verb::Attack => cmd_attack(cmd.object.as_deref(), i18n),
        Verb::Put => println!("\nPut what in what?"),
        Verb::Unknown(v) => {
            println!("\n{}", i18n.format(&ui.unknown_command, &[("cmd", &v)]));
        }
    }
}

fn cmd_look(state: &GameState, world: &World, i18n: &I18n) {
    let room_id = &state.current_room;

    if let Some(room_trans) = i18n.room(room_id) {
        println!("\n{}\n", room_trans.name);
        println!("{}\n", room_trans.description);
    } else {
        let room = world.get_room(room_id);
        println!("\n{}\n", room.name);
    }

    let objects = world.objects_in_room(room_id);
    for obj in objects {
        if let Some(obj_trans) = i18n.object(&obj.id) {
            println!("{}", obj_trans.description);
        }
    }
}

fn cmd_inventory(state: &GameState, ui: &crate::i18n::UiStrings) {
    if state.inventory.is_empty() {
        println!("\n{}", ui.empty_handed);
    } else {
        println!("\n{}:", ui.carrying);
        for item in &state.inventory {
            println!("  - {}", item);
        }
    }
}

fn cmd_move(state: &mut GameState, world: &mut World, verb: &Verb, i18n: &I18n) {
    let direction = match verb {
        Verb::North => "north",
        Verb::South => "south",
        Verb::East => "east",
        Verb::West => "west",
        Verb::Up => "up",
        Verb::Down => "down",
        _ => return,
    };

    let room = world.get_room(&state.current_room);

    if let Some(new_room) = room.exits.get(direction) {
        state.move_to(new_room);

        let room_id = &state.current_room;
        if let Some(room_trans) = i18n.room(room_id) {
            println!("\n{}\n", room_trans.name);
            println!("{}\n", room_trans.description);
        } else {
            let new_room_obj = world.get_room(room_id);
            println!("\n{}\n", new_room_obj.name);
        }
    } else {
        println!("\n{}", i18n.ui().cant_go);
    }
}

fn cmd_take(state: &mut GameState, world: &mut World, object: Option<&str>, i18n: &I18n) {
    let ui = i18n.ui();

    match object {
        Some(obj_name) => {
            let obj_id = find_object_by_name(world, &state.current_room, obj_name);

            if let Some(id) = obj_id {
                if let Some(obj) = world.get_object(&id) {
                    if !obj.takeable {
                        println!("\n{}", ui.cant_take);
                        return;
                    }
                }
                world.move_object(&id, "inventory");
                state.add_to_inventory(id.clone());
                println!("\n{}", ui.taken);
            } else {
                println!("\n{}", ui.dont_see);
            }
        }
        None => println!("\n{}", ui.take_what),
    }
}

fn cmd_drop(state: &mut GameState, world: &mut World, object: Option<&str>, i18n: &I18n) {
    let ui = i18n.ui();

    match object {
        Some(obj_name) => {
            let obj_id = state
                .inventory
                .iter()
                .find(|i| i.to_lowercase() == obj_name.to_lowercase())
                .cloned();

            if let Some(id) = obj_id {
                world.move_object(&id, &state.current_room);
                state.remove_from_inventory(&id);
                println!("\n{}", ui.dropped);
            } else {
                println!("\n{}", ui.not_holding);
            }
        }
        None => println!("\n{}", ui.drop_what),
    }
}

fn cmd_examine(state: &GameState, world: &World, object: Option<&str>, i18n: &I18n) {
    let ui = i18n.ui();

    match object {
        Some(obj_name) => {
            let obj_id = find_object_by_name(world, &state.current_room, obj_name).or_else(|| {
                state
                    .inventory
                    .iter()
                    .find(|i| i.to_lowercase() == obj_name.to_lowercase())
                    .cloned()
            });

            if let Some(id) = obj_id {
                if let Some(obj_trans) = i18n.object(&id) {
                    println!("\n{}", obj_trans.description);
                } else if let Some(obj) = world.get_object(&id) {
                    println!("\n{}", obj.name);
                }
            } else {
                println!("\n{}", ui.dont_see);
            }
        }
        None => println!("\n{}", ui.examine_what),
    }
}

fn cmd_open(state: &GameState, world: &mut World, object: Option<&str>, i18n: &I18n) {
    let ui = i18n.ui();

    match object {
        Some(obj_name) => {
            let obj_id = find_object_by_name(world, &state.current_room, obj_name);

            if let Some(id) = obj_id {
                if let Some(obj) = world.get_object_mut(&id) {
                    if !obj.is_openable {
                        println!("\n{}", ui.cant_open);
                    } else if obj.is_open {
                        println!("\n{}", ui.already_open);
                    } else {
                        obj.is_open = true;
                        println!("\n{}", ui.opened);
                    }
                }
            } else {
                println!("\n{}", ui.dont_see);
            }
        }
        None => println!("\n{}", ui.open_what),
    }
}

fn cmd_close(state: &GameState, world: &mut World, object: Option<&str>, i18n: &I18n) {
    let ui = i18n.ui();

    match object {
        Some(obj_name) => {
            let obj_id = find_object_by_name(world, &state.current_room, obj_name);

            if let Some(id) = obj_id {
                if let Some(obj) = world.get_object_mut(&id) {
                    if !obj.is_openable {
                        println!("\n{}", ui.cant_close);
                    } else if !obj.is_open {
                        println!("\n{}", ui.already_closed);
                    } else {
                        obj.is_open = false;
                        println!("\n{}", ui.closed);
                    }
                }
            } else {
                println!("\n{}", ui.dont_see);
            }
        }
        None => println!("\n{}", ui.close_what),
    }
}

fn cmd_read(state: &GameState, world: &World, object: Option<&str>, i18n: &I18n) {
    let ui = i18n.ui();

    match object {
        Some(obj_name) => {
            let obj_id = find_object_by_name(world, &state.current_room, obj_name).or_else(|| {
                state
                    .inventory
                    .iter()
                    .find(|i| i.to_lowercase() == obj_name.to_lowercase())
                    .cloned()
            });

            if let Some(id) = obj_id {
                if let Some(obj_trans) = i18n.object(&id) {
                    if let Some(read_text) = &obj_trans.read {
                        println!("\n{}", read_text);
                    } else {
                        println!("\n{}", ui.cant_read);
                    }
                } else {
                    println!("\n{}", ui.cant_read);
                }
            } else {
                println!("\n{}", ui.dont_see);
            }
        }
        None => println!("\n{}", ui.cant_read),
    }
}

fn cmd_use(state: &mut GameState, object: Option<&str>, i18n: &I18n) {
    let ui = i18n.ui();

    match object {
        Some(obj_name) => {
            if obj_name == "lamp" || obj_name == "lantern" || obj_name == "lanterna" {
                state.lamp_lit = !state.lamp_lit;
                if state.lamp_lit {
                    println!("\n{}", ui.lamp_on);
                } else {
                    println!("\n{}", ui.lamp_off);
                }
            } else {
                println!("\n{}", ui.unknown_command.replace("{cmd}", obj_name));
            }
        }
        None => println!("\nUse what?"),
    }
}

fn cmd_attack(object: Option<&str>, i18n: &I18n) {
    let ui = i18n.ui();

    match object {
        Some(target) => {
            println!(
                "\n{}",
                i18n.format(&ui.attack_with, &[("target", target), ("weapon", "fists")])
            );
        }
        None => println!("\n{}", ui.attack_what),
    }
}

fn find_object_by_name(world: &World, room_id: &str, name: &str) -> Option<String> {
    let objects = world.objects_in_room(room_id);
    for obj in objects {
        if obj.name.to_lowercase() == name.to_lowercase() {
            return Some(obj.id.clone());
        }
    }
    None
}
