use crate::game::state::GameState;
use crate::game::world::{CreatureState, World};
use crate::i18n::{I18n, Language};
use crate::logging;
use crate::parser::Command;
use crate::parser::Verb;

pub fn execute(state: &mut GameState, world: &mut World, cmd: Command, i18n: &I18n) {
    state.moves += 1;
    let ui = i18n.ui();
    let verb_dbg = format!("{:?}", cmd.verb);
    let object_dbg = cmd.object.clone().unwrap_or_default();
    logging::info(format!(
        "command.exec room={} moves={} verb={} object={}",
        state.current_room, state.moves, verb_dbg, object_dbg
    ));

    match cmd.verb {
        Verb::Look => cmd_look(state, world, i18n),
        Verb::Inventory => cmd_inventory(state, world, i18n),
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
            Ok(()) => {
                logging::info("save.ok slot=1");
                println!("\n{}", ui.game_saved);
            }
            Err(e) => {
                logging::error(format!("save.failed slot=1 error={}", e));
                println!("\n{}: {}", ui.save_failed, e);
            }
        },
        Verb::Restore => cmd_restore(state, world, i18n),
        Verb::Attack => cmd_attack(state, world, cmd.object.as_deref(), i18n),
        Verb::Put => cmd_put(state, world, cmd.object.as_deref(), i18n),
        Verb::Unknown(v) => {
            logging::warn(format!("command.unknown raw={}", v));
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

    let creatures = world.creatures_in_room(room_id);
    for creature in creatures {
        println!("{}", creature_display_line(creature, i18n));
    }

    let objects = world.objects_in_room(room_id);
    for obj in objects {
        if let Some(line) = object_display_line(world, i18n, &obj.id) {
            println!("{}", line);
        }

        if obj.is_openable && obj.is_open && !obj.contents.is_empty() {
            println!("  {}:", i18n.ui().contains);
            for content_id in &obj.contents.clone() {
                println!("    - {}", object_display_name(world, i18n, content_id));
            }
        }
    }
}

fn cmd_inventory(state: &GameState, world: &World, i18n: &I18n) {
    let ui = i18n.ui();

    if state.inventory.is_empty() {
        println!("\n{}", ui.empty_handed);
    } else {
        println!("\n{}:", ui.carrying);
        for item_id in &state.inventory {
            if world.get_object(item_id).is_some() {
                println!("  - {}", object_display_name(world, i18n, item_id));
            } else {
                println!("  - {}", item_id);
            }
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

    let current_room = state.current_room.clone();
    let room = world.get_room(&current_room);

    if let Some(new_room) = room.exits.get(direction) {
        if let Some(blocker) = world.blocking_creature(&current_room, new_room) {
            logging::warn(format!(
                "move.blocked from={} to={} by={}",
                current_room, new_room, blocker.id
            ));
            if let Some(creature_trans) = i18n.creature(&blocker.id) {
                println!("\n{}", creature_trans.description);
            } else {
                println!("\n{}", i18n.ui().cant_go);
            }
            return;
        }

        state.move_to(new_room);

        let room_id = &state.current_room;
        if let Some(room_trans) = i18n.room(room_id) {
            println!("\n{}\n", room_trans.name);
            println!("{}\n", room_trans.description);
        } else {
            let new_room_obj = world.get_room(room_id);
            println!("\n{}\n", new_room_obj.name);
        }
        logging::info(format!("move.ok from={} to={}", current_room, new_room));
    } else {
        logging::warn(format!(
            "move.invalid from={} direction={}",
            current_room, direction
        ));
        println!("\n{}", i18n.ui().cant_go);
    }
}

fn cmd_take(state: &mut GameState, world: &mut World, object: Option<&str>, i18n: &I18n) {
    let ui = i18n.ui();

    match object {
        Some(obj_name) => {
            let obj_id = find_object_by_name(world, &state.current_room, i18n, obj_name);

            if let Some(id) = obj_id {
                if let Some(obj) = world.get_object(&id) {
                    if !obj.takeable {
                        println!("\n{}", ui.cant_take);
                        return;
                    }
                }
                remove_from_container(world, &id);
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

fn remove_from_container(world: &mut World, object_id: &str) {
    for (_, obj) in world.objects.iter_mut() {
        obj.contents.retain(|id| id != object_id);
    }
}

fn cmd_drop(state: &mut GameState, world: &mut World, object: Option<&str>, i18n: &I18n) {
    let ui = i18n.ui();

    match object {
        Some(obj_name) => {
            let obj_id = find_inventory_object_by_name(state, world, i18n, obj_name);

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
            let obj_id = find_object_by_name(world, &state.current_room, i18n, obj_name)
                .or_else(|| find_inventory_object_by_name(state, world, i18n, obj_name));

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
            let obj_id = find_object_by_name(world, &state.current_room, i18n, obj_name);

            if let Some(id) = obj_id {
                if let Some(obj) = world.get_object_mut(&id) {
                    if !obj.is_openable {
                        println!("\n{}", ui.cant_open);
                    } else if obj.is_open {
                        println!("\n{}", ui.already_open);
                    } else {
                        obj.is_open = true;
                        println!("\n{}", ui.opened);
                        let contents = obj.contents.clone();
                        if !contents.is_empty() {
                            println!("\n{}", ui.contains);
                            for content_id in &contents {
                                println!("  - {}", object_display_name(world, i18n, content_id));
                            }
                        }
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
            let obj_id = find_object_by_name(world, &state.current_room, i18n, obj_name);

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
            let obj_id = find_object_by_name(world, &state.current_room, i18n, obj_name)
                .or_else(|| find_inventory_object_by_name(state, world, i18n, obj_name));

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
            let normalized = normalize_text(obj_name);
            let known = ["lamp", "lantern", "lanterna", "linterna"];

            if known.iter().any(|item| *item == normalized) {
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
        None => println!("\n{}", use_prompt(i18n.language())),
    }
}

fn cmd_attack(state: &mut GameState, world: &mut World, object: Option<&str>, i18n: &I18n) {
    let ui = i18n.ui();

    let Some(target_input) = object else {
        println!("\n{}", ui.attack_what);
        return;
    };

    let Some(target_id) = find_creature_by_name(world, &state.current_room, i18n, target_input)
    else {
        println!("\n{}", ui.dont_see);
        return;
    };

    let target_name = creature_display_name(world, i18n, &target_id);

    let Some((weapon_name, damage)) = best_weapon(state, world, i18n) else {
        println!(
            "\n{}",
            i18n.format(
                &ui.attack_with,
                &[
                    ("target", &target_name),
                    ("weapon", fists_name(i18n.language()))
                ],
            )
        );
        return;
    };

    println!(
        "\n{}",
        i18n.format(
            &ui.attack_with,
            &[("target", &target_name), ("weapon", &weapon_name)],
        )
    );

    let was_alive = world
        .get_creature(&target_id)
        .map(|c| c.is_alive())
        .unwrap_or(false);

    if !was_alive {
        if let Some(creature) = world.get_creature(&target_id) {
            println!("\n{}", creature_display_line(creature, i18n));
        }
        return;
    }

    let mut killed = false;
    if let Some(creature) = world.get_creature_mut(&target_id) {
        creature.take_damage(damage);
        killed = creature.state == CreatureState::Dead;
    }

    if killed {
        state.add_score(10);
        drop_items_from_creature(world, &target_id, &state.current_room);
        println!("\n{}", i18n.format(&ui.killed, &[("target", &target_name)]));
        if let Some(creature) = world.get_creature(&target_id) {
            println!("{}", creature_display_line(creature, i18n));
        }
        return;
    }

    if let Some(attacks_text) = i18n.creature(&target_id).and_then(|c| c.attacks.as_deref()) {
        println!("\n{}", attacks_text);
    } else {
        println!("\n{}", i18n.format(&ui.dodged, &[("target", &target_name)]));
    }
}

fn cmd_put(state: &mut GameState, world: &mut World, object: Option<&str>, i18n: &I18n) {
    let ui = i18n.ui();

    let Some(spec) = object else {
        println!("\n{}", put_prompt(i18n.language()));
        return;
    };

    let Some((item_name, container_name)) = parse_put_spec(spec, i18n.language()) else {
        println!("\n{}", put_prompt(i18n.language()));
        return;
    };

    let Some(item_id) = find_inventory_object_by_name(state, world, i18n, &item_name) else {
        println!("\n{}", ui.not_holding);
        return;
    };

    let container_id = find_object_by_name(world, &state.current_room, i18n, &container_name)
        .or_else(|| find_inventory_object_by_name(state, world, i18n, &container_name));

    let Some(container_id) = container_id else {
        println!("\n{}", ui.dont_see);
        return;
    };

    if item_id == container_id {
        println!("\n{}", ui.cant_put_in);
        return;
    }

    let can_put = world
        .get_object(&container_id)
        .map(|obj| obj.is_openable && obj.is_open)
        .unwrap_or(false);

    if !can_put {
        println!("\n{}", ui.cant_put_in);
        return;
    }

    remove_from_container(world, &item_id);
    world.move_object(&item_id, &container_id);
    state.remove_from_inventory(&item_id);

    if let Some(container) = world.get_object_mut(&container_id) {
        if !container.contents.contains(&item_id) {
            container.contents.push(item_id.clone());
        }
    }

    let item_disp = object_display_name(world, i18n, &item_id);
    let container_disp = object_display_name(world, i18n, &container_id);
    println!(
        "\n{}",
        i18n.format(
            &ui.put_in,
            &[("obj", &item_disp), ("container", &container_disp)],
        )
    );
}

fn cmd_restore(state: &mut GameState, world: &World, i18n: &I18n) {
    let ui = i18n.ui();

    match GameState::load(1) {
        Ok(loaded) => {
            *state = loaded;
            logging::info("restore.ok slot=1");
            println!("\n{}", ui.game_restored);
            cmd_look(state, world, i18n);
        }
        Err(e) if e == "No saved game found" => {
            logging::warn("restore.missing slot=1");
            println!("\n{}", ui.no_saved_game);
        }
        Err(e) => {
            logging::error(format!("restore.failed slot=1 error={}", e));
            println!("\n{}: {}", ui.restore_failed, e);
        }
    }
}

fn use_prompt(lang: Language) -> &'static str {
    match lang {
        Language::English => "Use what?",
        Language::Italian => "Usare cosa?",
        Language::Spanish => "Usar que?",
    }
}

fn put_prompt(lang: Language) -> &'static str {
    match lang {
        Language::English => "Put what in what?",
        Language::Italian => "Mettere cosa in cosa?",
        Language::Spanish => "Poner que en que?",
    }
}

fn parse_put_spec(spec: &str, lang: Language) -> Option<(String, String)> {
    let separators = match lang {
        Language::English => vec![" into ", " in ", " inside "],
        Language::Italian => vec![
            " dentro ", " nella ", " nello ", " negli ", " nelle ", " nel ", " nei ", " in ",
        ],
        Language::Spanish => vec![" dentro ", " en "],
    };

    let normalized_spec = normalize_text(spec);

    for sep in separators {
        let sep_norm = normalize_text(sep);
        if let Some((left, right)) = normalized_spec.split_once(&sep_norm) {
            let item = left.trim().to_string();
            let container = right.trim().to_string();
            if !item.is_empty() && !container.is_empty() {
                return Some((item, container));
            }
        }
    }

    None
}

fn find_creature_by_name(world: &World, room_id: &str, i18n: &I18n, name: &str) -> Option<String> {
    world
        .creatures_in_room(room_id)
        .into_iter()
        .find(|creature| creature_matches_input(i18n, creature.id.as_str(), &creature.name, name))
        .map(|creature| creature.id.clone())
}

fn creature_matches_input(i18n: &I18n, creature_id: &str, world_name: &str, input: &str) -> bool {
    let mut aliases: Vec<String> = vec![creature_id.to_string(), world_name.to_string()];

    if let Some(trans) = i18n.creature(creature_id) {
        aliases.push(trans.name.clone());
    }

    if let Some(obj_trans) = i18n.object(creature_id) {
        aliases.push(obj_trans.name.clone());
    }

    aliases.iter().any(|alias| is_name_match(alias, input))
}

fn creature_display_name(world: &World, i18n: &I18n, creature_id: &str) -> String {
    if let Some(trans) = i18n.creature(creature_id) {
        return trans.name.clone();
    }

    if let Some(creature) = world.get_creature(creature_id) {
        return creature.name.clone();
    }

    creature_id.to_string()
}

fn creature_display_line(creature: &crate::game::world::Creature, i18n: &I18n) -> String {
    if let Some(trans) = i18n.creature(&creature.id) {
        if creature.state == CreatureState::Dead {
            if let Some(dead_desc) = trans.dead_desc.as_ref() {
                return dead_desc.clone();
            }
            return trans.description.clone();
        }
        return trans.description.clone();
    }

    if creature.state == CreatureState::Dead {
        return format!("The dead body of {} lies here.", creature.name);
    }

    format!("There is a {} here.", creature.name)
}

fn drop_items_from_creature(world: &mut World, creature_id: &str, room_id: &str) {
    let room = room_id.to_string();
    for location in world.object_locations.values_mut() {
        if location == creature_id {
            *location = room.clone();
        }
    }
}

fn best_weapon(state: &GameState, world: &World, i18n: &I18n) -> Option<(String, i32)> {
    let weapon_priority = [("sword", 3), ("axe", 3), ("stiletto", 2), ("knife", 2)];

    for (weapon_id, damage) in weapon_priority {
        if state.inventory.iter().any(|item| item == weapon_id) {
            return Some((object_display_name(world, i18n, weapon_id), damage));
        }
    }

    None
}

fn fists_name(lang: Language) -> &'static str {
    match lang {
        Language::English => "fists",
        Language::Italian => "pugni",
        Language::Spanish => "punos",
    }
}

fn find_object_by_name(world: &World, room_id: &str, i18n: &I18n, name: &str) -> Option<String> {
    let objects = world.objects_in_room(room_id);

    for obj in objects {
        if object_matches_input(world, i18n, &obj.id, name) {
            return Some(obj.id.clone());
        }

        if obj.is_openable && obj.is_open {
            for content_id in &obj.contents.clone() {
                if world.get_object(content_id).is_some()
                    && object_matches_input(world, i18n, content_id, name)
                {
                    return Some(content_id.clone());
                }
            }
        }
    }

    None
}

fn find_inventory_object_by_name(
    state: &GameState,
    world: &World,
    i18n: &I18n,
    name: &str,
) -> Option<String> {
    state
        .inventory
        .iter()
        .find(|id| object_matches_input(world, i18n, id, name))
        .cloned()
}

fn object_matches_input(world: &World, i18n: &I18n, object_id: &str, input: &str) -> bool {
    let mut aliases: Vec<String> = vec![object_id.to_string()];

    if let Some(obj) = world.get_object(object_id) {
        aliases.push(obj.name.clone());
    }

    if let Some(trans) = i18n.object(object_id) {
        aliases.push(trans.name.clone());
    }

    aliases.iter().any(|alias| is_name_match(alias, input))
}

fn is_name_match(alias: &str, input: &str) -> bool {
    let normalized_alias = normalize_text(alias);
    let normalized_input = normalize_text(input);

    if normalized_alias.is_empty() || normalized_input.is_empty() {
        return false;
    }

    normalized_alias == normalized_input
        || normalized_alias.contains(&normalized_input)
        || normalized_input.contains(&normalized_alias)
}

fn normalize_text(value: &str) -> String {
    let mapped = value
        .to_lowercase()
        .chars()
        .map(|ch| match ch {
            'à' | 'á' | 'â' | 'ä' | 'ã' => 'a',
            'è' | 'é' | 'ê' | 'ë' => 'e',
            'ì' | 'í' | 'î' | 'ï' => 'i',
            'ò' | 'ó' | 'ô' | 'ö' | 'õ' => 'o',
            'ù' | 'ú' | 'û' | 'ü' => 'u',
            'ñ' => 'n',
            '\'' | '"' | '.' | ',' | ';' | ':' | '-' | '_' | '/' | '\\' | '(' | ')' => ' ',
            _ if ch.is_alphanumeric() || ch.is_whitespace() => ch,
            _ => ' ',
        })
        .collect::<String>();

    mapped.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn object_display_name(world: &World, i18n: &I18n, object_id: &str) -> String {
    if let Some(obj_trans) = i18n.object(object_id) {
        return obj_trans.name.clone();
    }

    if let Some(obj) = world.get_object(object_id) {
        return obj.name.clone();
    }

    object_id.to_string()
}

fn object_display_line(world: &World, i18n: &I18n, object_id: &str) -> Option<String> {
    let lang = i18n.language();

    if let Some(obj_trans) = i18n.object(object_id) {
        if should_fallback_from_description(i18n.language(), &obj_trans.description) {
            if lang != Language::English
                && looks_untranslated_name(world, object_id, &obj_trans.name)
            {
                return None;
            }

            return Some(obj_trans.name.clone());
        }

        return Some(obj_trans.description.clone());
    }

    if lang != Language::English {
        return None;
    }

    Some(object_display_name(world, i18n, object_id))
}

fn should_fallback_from_description(lang: Language, description: &str) -> bool {
    if lang == Language::English {
        return false;
    }

    let normalized = normalize_text(description);
    let english_prefixes = ["there is ", "there are ", "you see ", "you can see "];

    english_prefixes
        .iter()
        .any(|prefix| normalized.starts_with(prefix))
}

fn looks_untranslated_name(world: &World, object_id: &str, translated_name: &str) -> bool {
    let Some(obj) = world.get_object(object_id) else {
        return false;
    };

    let normalized_translated = normalize_text(translated_name);
    let normalized_world = normalize_text(&obj.name);
    let word_count = normalized_translated.split_whitespace().count();

    normalized_translated == normalized_world && word_count <= 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Language;
    use std::path::PathBuf;
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> &'static Mutex<()> {
        static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        ENV_LOCK.get_or_init(|| Mutex::new(()))
    }

    fn with_temp_home<T>(test_name: &str, f: impl FnOnce(PathBuf) -> T) -> T {
        let _guard = env_lock().lock().expect("env lock should be available");
        let original_home = std::env::var("HOME").ok();
        let temp_home =
            std::env::temp_dir().join(format!("zork-termux-{}-{}", test_name, std::process::id()));
        let _ = std::fs::remove_dir_all(&temp_home);
        std::fs::create_dir_all(&temp_home).expect("temp HOME should be created");
        unsafe {
            std::env::set_var("HOME", &temp_home);
        }

        let result = f(temp_home.clone());

        match original_home {
            Some(value) => unsafe {
                std::env::set_var("HOME", value);
            },
            None => unsafe {
                std::env::remove_var("HOME");
            },
        }

        let _ = std::fs::remove_dir_all(temp_home);
        result
    }

    #[test]
    fn troll_blocks_passage_until_killed() {
        let i18n = I18n::load(Language::English).expect("translation should load");
        let mut world = World::load_zork1();
        let mut state = GameState::new(Language::English, "ew_passage");

        execute(
            &mut state,
            &mut world,
            Command {
                verb: Verb::West,
                object: None,
            },
            &i18n,
        );
        assert_eq!(state.current_room, "ew_passage");

        state.current_room = "troll_room".to_string();
        state.add_to_inventory("sword".to_string());
        world.move_object("sword", "inventory");

        execute(
            &mut state,
            &mut world,
            Command {
                verb: Verb::Attack,
                object: Some("troll".to_string()),
            },
            &i18n,
        );

        let troll = world
            .get_creature("troll")
            .expect("troll creature should exist");
        assert_eq!(troll.state, CreatureState::Dead);
        assert_eq!(world.object_location("axe"), Some("troll_room"));

        state.current_room = "ew_passage".to_string();
        execute(
            &mut state,
            &mut world,
            Command {
                verb: Verb::West,
                object: None,
            },
            &i18n,
        );
        assert_eq!(state.current_room, "troll_room");
    }

    #[test]
    fn save_restore_roundtrip_via_commands() {
        with_temp_home("save-restore", |_| {
            let i18n = I18n::load(Language::English).expect("translation should load");
            let mut world = World::load_zork1();
            let mut state = GameState::new(Language::English, "west_of_house");

            execute(
                &mut state,
                &mut world,
                Command {
                    verb: Verb::Save,
                    object: None,
                },
                &i18n,
            );

            assert!(GameState::save_exists(1));

            state.current_room = "north_of_house".to_string();
            state.score = 77;
            state.add_to_inventory("sword".to_string());

            execute(
                &mut state,
                &mut world,
                Command {
                    verb: Verb::Restore,
                    object: None,
                },
                &i18n,
            );

            assert_eq!(state.current_room, "west_of_house");
            assert_eq!(state.score, 0);
            assert!(state.inventory.is_empty());
            assert_eq!(state.moves, 1);
        });
    }

    #[test]
    fn put_moves_item_into_open_container() {
        let i18n = I18n::load(Language::English).expect("translation should load");
        let mut world = World::load_zork1();
        let mut state = GameState::new(Language::English, "west_of_house");

        execute(
            &mut state,
            &mut world,
            Command {
                verb: Verb::Open,
                object: Some("mailbox".to_string()),
            },
            &i18n,
        );
        execute(
            &mut state,
            &mut world,
            Command {
                verb: Verb::Take,
                object: Some("leaflet".to_string()),
            },
            &i18n,
        );
        assert!(state.inventory.contains(&"advertisement".to_string()));

        execute(
            &mut state,
            &mut world,
            Command {
                verb: Verb::Put,
                object: Some("leaflet in mailbox".to_string()),
            },
            &i18n,
        );

        assert!(!state.inventory.contains(&"advertisement".to_string()));
        assert_eq!(world.object_location("advertisement"), Some("mailbox"));
        let mailbox = world
            .get_object("mailbox")
            .expect("mailbox object should exist");
        assert!(mailbox.contents.iter().any(|id| id == "advertisement"));
    }
}
