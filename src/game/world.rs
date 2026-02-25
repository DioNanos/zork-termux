use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub exits: HashMap<String, String>,
    pub is_dark: bool,
}

impl Room {
    pub fn new(id: &str, name: &str) -> Self {
        Room {
            id: id.to_string(),
            name: name.to_string(),
            exits: HashMap::new(),
            is_dark: false,
        }
    }

    pub fn with_exit(mut self, direction: &str, destination: &str) -> Self {
        self.exits
            .insert(direction.to_string(), destination.to_string());
        self
    }

    pub fn dark(mut self) -> Self {
        self.is_dark = true;
        self
    }
}

#[derive(Debug, Clone)]
pub struct Object {
    pub id: String,
    pub name: String,
    pub takeable: bool,
    pub is_openable: bool,
    pub is_open: bool,
    pub is_lit: bool,
    pub is_locked: bool,
    pub contents: Vec<String>,
}

impl Object {
    pub fn new(id: &str, name: &str) -> Self {
        Object {
            id: id.to_string(),
            name: name.to_string(),
            takeable: true,
            is_openable: false,
            is_open: false,
            is_lit: false,
            is_locked: false,
            contents: Vec::new(),
        }
    }

    pub fn takeable(mut self, takeable: bool) -> Self {
        self.takeable = takeable;
        self
    }

    pub fn openable(mut self) -> Self {
        self.is_openable = true;
        self
    }

    pub fn open(mut self) -> Self {
        self.is_open = true;
        self
    }

    pub fn lit(mut self) -> Self {
        self.is_lit = true;
        self
    }

    pub fn locked(mut self) -> Self {
        self.is_locked = true;
        self
    }

    pub fn with_content(mut self, content_id: &str) -> Self {
        self.contents.push(content_id.to_string());
        self
    }
}

pub struct World {
    pub rooms: HashMap<String, Room>,
    pub objects: HashMap<String, Object>,
    pub object_locations: HashMap<String, String>,
}

impl World {
    pub fn new() -> Self {
        World {
            rooms: HashMap::new(),
            objects: HashMap::new(),
            object_locations: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.insert(room.id.clone(), room);
    }

    pub fn add_object(&mut self, object: Object, location: &str) {
        let id = object.id.clone();
        self.objects.insert(id.clone(), object);
        self.object_locations.insert(id, location.to_string());
    }

    pub fn get_room(&self, id: &str) -> &Room {
        self.rooms.get(id).expect("Room not found")
    }

    pub fn get_room_mut(&mut self, id: &str) -> &mut Room {
        self.rooms.get_mut(id).expect("Room not found")
    }

    pub fn get_object(&self, id: &str) -> Option<&Object> {
        self.objects.get(id)
    }

    pub fn get_object_mut(&mut self, id: &str) -> Option<&mut Object> {
        self.objects.get_mut(id)
    }

    pub fn objects_in_room(&self, room_id: &str) -> Vec<&Object> {
        self.object_locations
            .iter()
            .filter(|(_, loc)| *loc == room_id)
            .filter_map(|(id, _)| self.objects.get(id))
            .collect()
    }

    pub fn move_object(&mut self, object_id: &str, new_location: &str) {
        if self.objects.contains_key(object_id) {
            self.object_locations
                .insert(object_id.to_string(), new_location.to_string());
        }
    }

    pub fn object_location(&self, object_id: &str) -> Option<&str> {
        self.object_locations.get(object_id).map(|s| s.as_str())
    }

    pub fn load_zork1() -> Self {
        let mut world = World::new();

        // ZORK1: 68 rooms, 121 objects

        world.add_room(
            Room::new("west_of_house", "West of House")
                .with_exit("north", "north_of_house")
                .with_exit("south", "south_of_house")
                .with_exit("ne", "north_of_house")
                .with_exit("se", "south_of_house")
                .with_exit("west", "forest_1"),
        );

        world.add_room(Room::new("stone_barrow", "Stone Barrow").with_exit("ne", "west_of_house"));

        world.add_room(
            Room::new("north_of_house", "North of House")
                .with_exit("sw", "west_of_house")
                .with_exit("se", "east_of_house")
                .with_exit("west", "west_of_house")
                .with_exit("east", "east_of_house")
                .with_exit("north", "path"),
        );

        world.add_room(
            Room::new("south_of_house", "South of House")
                .with_exit("west", "west_of_house")
                .with_exit("east", "east_of_house")
                .with_exit("ne", "east_of_house")
                .with_exit("nw", "west_of_house")
                .with_exit("south", "forest_3"),
        );

        world.add_room(
            Room::new("east_of_house", "Behind House")
                .with_exit("north", "north_of_house")
                .with_exit("south", "south_of_house")
                .with_exit("sw", "south_of_house")
                .with_exit("nw", "north_of_house")
                .with_exit("east", "clearing"),
        );

        world.add_room(
            Room::new("forest_1", "Forest")
                .with_exit("north", "grating_clearing")
                .with_exit("east", "path")
                .with_exit("south", "forest_3"),
        );

        world.add_room(
            Room::new("forest_2", "Forest")
                .with_exit("east", "mountains")
                .with_exit("south", "clearing")
                .with_exit("west", "path"),
        );

        world.add_room(
            Room::new("mountains", "Forest")
                .with_exit("north", "forest_2")
                .with_exit("south", "forest_2")
                .with_exit("west", "forest_2"),
        );

        world.add_room(
            Room::new("forest_3", "Forest")
                .with_exit("north", "clearing")
                .with_exit("west", "forest_1")
                .with_exit("nw", "south_of_house"),
        );

        world.add_room(
            Room::new("path", "Forest Path")
                .with_exit("up", "up_a_tree")
                .with_exit("north", "grating_clearing")
                .with_exit("east", "forest_2")
                .with_exit("south", "north_of_house")
                .with_exit("west", "forest_1"),
        );

        world.add_room(Room::new("up_a_tree", "Up a Tree").with_exit("down", "path"));

        world.add_room(
            Room::new("grating_clearing", "Clearing")
                .with_exit("east", "forest_2")
                .with_exit("west", "forest_1")
                .with_exit("south", "path"),
        );

        world.add_room(
            Room::new("clearing", "Clearing")
                .with_exit("east", "canyon_view")
                .with_exit("north", "forest_2")
                .with_exit("south", "forest_3")
                .with_exit("west", "east_of_house"),
        );

        world.add_room(
            Room::new("kitchen", "Kitchen")
                .with_exit("west", "living_room")
                .with_exit("up", "attic"),
        );

        world.add_room(Room::new("attic", "Attic").with_exit("down", "kitchen"));

        world.add_room(Room::new("living_room", "Living Room").with_exit("east", "kitchen"));

        world.add_room(
            Room::new("cellar", "Cellar")
                .with_exit("north", "troll_room")
                .with_exit("south", "east_of_chasm"),
        );

        world.add_room(Room::new("troll_room", "The Troll Room").with_exit("south", "cellar"));

        world.add_room(
            Room::new("east_of_chasm", "East of Chasm")
                .with_exit("north", "cellar")
                .with_exit("east", "gallery"),
        );

        world.add_room(
            Room::new("gallery", "Gallery")
                .with_exit("west", "east_of_chasm")
                .with_exit("north", "studio"),
        );

        world.add_room(Room::new("studio", "Studio").with_exit("south", "gallery"));

        world.add_room(
            Room::new("maze_1", "Maze")
                .with_exit("east", "troll_room")
                .with_exit("north", "maze_1")
                .with_exit("south", "maze_2")
                .with_exit("west", "maze_4"),
        );

        world.add_room(
            Room::new("maze_2", "Maze")
                .with_exit("south", "maze_1")
                .with_exit("east", "maze_3"),
        );

        world.add_room(
            Room::new("maze_3", "Maze")
                .with_exit("west", "maze_2")
                .with_exit("north", "maze_4")
                .with_exit("up", "maze_5"),
        );

        world.add_room(
            Room::new("maze_4", "Maze")
                .with_exit("west", "maze_3")
                .with_exit("north", "maze_1")
                .with_exit("east", "dead_end_1"),
        );

        world.add_room(Room::new("dead_end_1", "Dead End").with_exit("south", "maze_4"));

        world.add_room(
            Room::new("maze_5", "Maze")
                .with_exit("east", "dead_end_2")
                .with_exit("north", "maze_3")
                .with_exit("sw", "maze_6"),
        );

        world.add_room(Room::new("dead_end_2", "Dead End").with_exit("west", "maze_5"));

        world.add_room(
            Room::new("maze_6", "Maze")
                .with_exit("down", "maze_5")
                .with_exit("east", "maze_7")
                .with_exit("west", "maze_6")
                .with_exit("up", "maze_9"),
        );

        world.add_room(
            Room::new("maze_7", "Maze")
                .with_exit("up", "maze_14")
                .with_exit("west", "maze_6")
                .with_exit("east", "maze_8")
                .with_exit("south", "maze_15"),
        );

        world.add_room(
            Room::new("maze_8", "Maze")
                .with_exit("ne", "maze_7")
                .with_exit("west", "maze_8")
                .with_exit("se", "dead_end_3"),
        );

        world.add_room(Room::new("dead_end_3", "Dead End").with_exit("north", "maze_8"));

        world.add_room(
            Room::new("maze_9", "Maze")
                .with_exit("north", "maze_6")
                .with_exit("east", "maze_10")
                .with_exit("south", "maze_13")
                .with_exit("west", "maze_12")
                .with_exit("nw", "maze_9"),
        );

        world.add_room(
            Room::new("maze_10", "Maze")
                .with_exit("east", "maze_9")
                .with_exit("west", "maze_13")
                .with_exit("up", "maze_11"),
        );

        world.add_room(
            Room::new("maze_11", "Maze")
                .with_exit("ne", "grating_room")
                .with_exit("down", "maze_10")
                .with_exit("nw", "maze_13")
                .with_exit("sw", "maze_12"),
        );

        world.add_room(Room::new("grating_room", "Grating Room").with_exit("sw", "maze_11"));

        world.add_room(
            Room::new("maze_12", "Maze")
                .with_exit("sw", "maze_11")
                .with_exit("east", "maze_13")
                .with_exit("up", "maze_9")
                .with_exit("north", "dead_end_4"),
        );

        world.add_room(Room::new("dead_end_4", "Dead End").with_exit("south", "maze_12"));

        world.add_room(
            Room::new("maze_13", "Maze")
                .with_exit("east", "maze_9")
                .with_exit("down", "maze_12")
                .with_exit("south", "maze_10")
                .with_exit("west", "maze_11"),
        );

        world.add_room(
            Room::new("maze_14", "Maze")
                .with_exit("west", "maze_15")
                .with_exit("nw", "maze_14")
                .with_exit("ne", "maze_7")
                .with_exit("south", "maze_7"),
        );

        world.add_room(
            Room::new("maze_15", "Maze")
                .with_exit("west", "maze_14")
                .with_exit("south", "maze_7")
                .with_exit("se", "cyclops_room"),
        );

        world.add_room(Room::new("cyclops_room", "Cyclops Room").with_exit("nw", "maze_15"));

        world.add_room(
            Room::new("strange_passage", "Strange Passage")
                .with_exit("west", "cyclops_room")
                .with_exit("in", "cyclops_room")
                .with_exit("east", "living_room"),
        );

        world.add_room(
            Room::new("treasure_room", "Treasure Room").with_exit("down", "cyclops_room"),
        );

        world.add_room(
            Room::new("reservoir_south", "Reservoir South")
                .with_exit("se", "deep_canyon")
                .with_exit("sw", "chasm_room")
                .with_exit("east", "dam_room")
                .with_exit("west", "stream_view"),
        );

        world.add_room(
            Room::new("reservoir", "Reservoir")
                .with_exit("north", "reservoir_north")
                .with_exit("south", "reservoir_south")
                .with_exit("up", "in_stream")
                .with_exit("west", "in_stream"),
        );

        world.add_room(
            Room::new("reservoir_north", "Reservoir North").with_exit("north", "atlantis_room"),
        );

        world
            .add_room(Room::new("stream_view", "Stream View").with_exit("east", "reservoir_south"));

        world.add_room(
            Room::new("in_stream", "Stream")
                .with_exit("land", "stream_view")
                .with_exit("down", "reservoir")
                .with_exit("east", "reservoir"),
        );

        world.add_room(
            Room::new("mirror_room_1", "Mirror Room")
                .with_exit("north", "cold_passage")
                .with_exit("west", "twisting_passage")
                .with_exit("east", "small_cave"),
        );

        world.add_room(
            Room::new("mirror_room_2", "Mirror Room")
                .with_exit("west", "winding_passage")
                .with_exit("north", "narrow_passage")
                .with_exit("east", "tiny_cave"),
        );

        world.add_room(
            Room::new("small_cave", "Cave")
                .with_exit("north", "mirror_room_1")
                .with_exit("down", "atlantis_room")
                .with_exit("south", "atlantis_room")
                .with_exit("west", "twisting_passage"),
        );

        world.add_room(
            Room::new("tiny_cave", "Cave")
                .with_exit("north", "mirror_room_2")
                .with_exit("west", "winding_passage")
                .with_exit("down", "entrance_to_hades"),
        );

        world.add_room(
            Room::new("cold_passage", "Cold Passage")
                .with_exit("south", "mirror_room_1")
                .with_exit("west", "slide_room"),
        );

        world.add_room(
            Room::new("narrow_passage", "Narrow Passage")
                .with_exit("north", "round_room")
                .with_exit("south", "mirror_room_2"),
        );

        world.add_room(
            Room::new("winding_passage", "Winding Passage")
                .with_exit("north", "mirror_room_2")
                .with_exit("east", "tiny_cave"),
        );

        world.add_room(
            Room::new("twisting_passage", "Twisting Passage")
                .with_exit("north", "mirror_room_1")
                .with_exit("east", "small_cave"),
        );

        world.add_room(
            Room::new("atlantis_room", "Atlantis Room")
                .with_exit("up", "small_cave")
                .with_exit("south", "reservoir_north"),
        );

        world.add_room(
            Room::new("ew_passage", "East-West Passage")
                .with_exit("east", "round_room")
                .with_exit("west", "troll_room")
                .with_exit("down", "chasm_room")
                .with_exit("north", "chasm_room"),
        );

        world.add_room(
            Room::new("round_room", "Round Room")
                .with_exit("east", "loud_room")
                .with_exit("west", "ew_passage")
                .with_exit("north", "ns_passage")
                .with_exit("south", "narrow_passage")
                .with_exit("se", "engravings_cave"),
        );

        world.add_room(
            Room::new("deep_canyon", "Deep Canyon")
                .with_exit("nw", "reservoir_south")
                .with_exit("east", "dam_room")
                .with_exit("sw", "ns_passage")
                .with_exit("down", "loud_room"),
        );

        world.add_room(
            Room::new("damp_cave", "Damp Cave")
                .with_exit("west", "loud_room")
                .with_exit("east", "white_cliffs_north"),
        );

        world.add_room(
            Room::new("loud_room", "Loud Room")
                .with_exit("east", "damp_cave")
                .with_exit("west", "round_room")
                .with_exit("up", "deep_canyon"),
        );

        world.add_room(
            Room::new("ns_passage", "North-South Passage")
                .with_exit("north", "chasm_room")
                .with_exit("ne", "deep_canyon")
                .with_exit("south", "round_room"),
        );

        world.add_room(
            Room::new("chasm_room", "Chasm")
                .with_exit("ne", "reservoir_south")
                .with_exit("sw", "ew_passage")
                .with_exit("up", "ew_passage")
                .with_exit("south", "ns_passage"),
        );

        world.add_room(
            Room::new("entrance_to_hades", "Entrance to Hades").with_exit("up", "tiny_cave"),
        );

        world.add_room(
            Room::new("land_of_living_dead", "Land of the Dead")
                .with_exit("out", "entrance_to_hades")
                .with_exit("north", "entrance_to_hades"),
        );

        world.add_room(
            Room::new("torch_room", "Torch Room")
                .with_exit("south", "north_temple")
                .with_exit("down", "north_temple"),
        );

        world.add_object(
            Object::new("mountain_range", "mountain range").takeable(false),
            "mountains",
        );

        world.add_object(Object::new("water", "quantity of water"), "bottle");

        world.add_object(
            Object::new("ghosts", "number of ghosts").takeable(false),
            "entrance_to_hades",
        );

        world.add_object(Object::new("skull", "crystal skull"), "land_of_living_dead");

        world.add_object(
            Object::new("lowered_basket", "basket").takeable(false),
            "lower_shaft",
        );

        world.add_object(
            Object::new("raised_basket", "basket")
                .takeable(false)
                .openable(),
            "shaft_room",
        );

        world.add_object(Object::new("lunch", "lunch"), "sandwich_bag");

        world.add_object(Object::new("bat", "bat").takeable(false), "bat_room");

        world.add_object(Object::new("bell", "brass bell"), "north_temple");

        world.add_object(Object::new("axe", "bloody axe"), "troll");

        world.add_object(Object::new("bolt", "bolt").takeable(false), "dam_room");

        world.add_object(
            Object::new("bubble", "green bubble").takeable(false),
            "dam_room",
        );

        world.add_object(
            Object::new("altar", "altar")
                .takeable(false)
                .openable()
                .with_content("book"),
            "south_temple",
        );

        world.add_object(Object::new("book", "black book").openable(), "altar");

        world.add_object(Object::new("sceptre", "sceptre"), "coffin");

        world.add_object(Object::new("timbers", "broken timber"), "timber_room");

        world.add_object(
            Object::new("kitchen_table", "kitchen table")
                .takeable(false)
                .openable(),
            "kitchen",
        );

        world.add_object(
            Object::new("attic_table", "table")
                .takeable(false)
                .openable(),
            "attic",
        );

        world.add_object(
            Object::new("sandwich_bag", "brown sack").openable(),
            "kitchen_table",
        );

        world.add_object(
            Object::new("tool_chest", "group of tool chests")
                .takeable(false)
                .openable(),
            "maintenance_room",
        );

        world.add_object(
            Object::new("yellow_button", "yellow button").takeable(false),
            "maintenance_room",
        );

        world.add_object(
            Object::new("brown_button", "brown button").takeable(false),
            "maintenance_room",
        );

        world.add_object(
            Object::new("red_button", "red button").takeable(false),
            "maintenance_room",
        );

        world.add_object(
            Object::new("blue_button", "blue button").takeable(false),
            "maintenance_room",
        );

        world.add_object(Object::new("rug", "carpet").takeable(false), "living_room");

        world.add_object(
            Object::new("chalice", "chalice").openable(),
            "treasure_room",
        );

        world.add_object(Object::new("garlic", "clove of garlic"), "sandwich_bag");

        world.add_object(Object::new("trident", "crystal trident"), "atlantis_room");

        world.add_object(
            Object::new("cyclops", "cyclops").takeable(false),
            "cyclops_room",
        );

        world.add_object(Object::new("dam", "dam").takeable(false), "dam_room");

        world.add_object(
            Object::new("trap_door", "trap door")
                .takeable(false)
                .openable(),
            "living_room",
        );

        world.add_object(
            Object::new("front_door", "door").takeable(false).openable(),
            "west_of_house",
        );

        world.add_object(
            Object::new("barrow_door", "stone door")
                .takeable(false)
                .openable(),
            "stone_barrow",
        );

        world.add_object(
            Object::new("barrow", "stone barrow").takeable(false),
            "stone_barrow",
        );

        world.add_object(
            Object::new("bottle", "glass bottle")
                .openable()
                .with_content("water"),
            "kitchen_table",
        );

        world.add_object(
            Object::new("coffin", "gold coffin")
                .openable()
                .with_content("sceptre"),
            "egypt_room",
        );

        world.add_object(Object::new("pump", "hand-held air pump"), "reservoir_north");

        world.add_object(Object::new("jade", "jade figurine"), "bat_room");

        world.add_object(Object::new("knife", "nasty knife"), "attic_table");

        world.add_object(Object::new("lamp", "brass lantern"), "living_room");

        world.add_object(Object::new("emerald", "large emerald"), "buoy");

        world.add_object(Object::new("advertisement", "leaflet"), "mailbox");

        world.add_object(
            Object::new("leak", "leak").takeable(false),
            "maintenance_room",
        );

        world.add_object(
            Object::new("machine", "machine").takeable(false).openable(),
            "machine_room",
        );

        world.add_object(
            Object::new("mailbox", "small mailbox")
                .takeable(false)
                .openable()
                .with_content("advertisement"),
            "west_of_house",
        );

        world.add_object(Object::new("match", "matchbook"), "dam_lobby");

        world.add_object(Object::new("painting", "painting"), "gallery");

        world.add_object(Object::new("candles", "pair of candles"), "south_temple");

        world.add_object(Object::new("leaves", "pile of leaves"), "grating_clearing");

        world.add_object(
            Object::new("inflatable_boat", "pile of plastic"),
            "dam_base",
        );

        world.add_object(Object::new("bar", "platinum bar"), "loud_room");

        world.add_object(Object::new("pot_of_gold", "pot of gold"), "end_of_rainbow");

        world.add_object(
            Object::new("prayer", "prayer").takeable(false),
            "north_temple",
        );

        world.add_object(
            Object::new("railing", "wooden railing").takeable(false),
            "dome_room",
        );

        world.add_object(Object::new("rope", "rope"), "attic");

        world.add_object(Object::new("sand", "sand").takeable(false), "sandy_cave");

        world.add_object(
            Object::new("bracelet", "sapphire-encrusted bracelet"),
            "gas_room",
        );

        world.add_object(
            Object::new("screwdriver", "screwdriver"),
            "maintenance_room",
        );

        world.add_object(Object::new("shovel", "shovel"), "sandy_beach");

        world.add_object(
            Object::new("scarab", "beautiful jeweled scarab"),
            "sandy_cave",
        );

        world.add_object(
            Object::new("large_bag", "large bag").takeable(false),
            "thief",
        );

        world.add_object(Object::new("stiletto", "stiletto"), "thief");

        world.add_object(
            Object::new("machine_switch", "switch").takeable(false),
            "machine_room",
        );

        world.add_object(
            Object::new("wooden_door", "wooden door")
                .takeable(false)
                .openable(),
            "living_room",
        );

        world.add_object(Object::new("sword", "sword"), "living_room");

        world.add_object(Object::new("map", "ancient map"), "trophy_case");

        world.add_object(Object::new("boat_label", "tan label"), "inflated_boat");

        world.add_object(
            Object::new("thief", "thief")
                .takeable(false)
                .openable()
                .with_content("stiletto"),
            "round_room",
        );

        world.add_object(
            Object::new("pedestal", "pedestal")
                .takeable(false)
                .openable(),
            "torch_room",
        );

        world.add_object(Object::new("torch", "torch"), "pedestal");

        world.add_object(Object::new("guide", "tour guidebook"), "dam_lobby");

        world.add_object(
            Object::new("troll", "troll").takeable(false).openable(),
            "troll_room",
        );

        world.add_object(Object::new("trunk", "trunk of jewels"), "reservoir");

        world.add_object(
            Object::new("tube", "tube").openable().with_content("putty"),
            "maintenance_room",
        );

        world.add_object(Object::new("putty", "viscous material"), "tube");

        world.add_object(
            Object::new("engravings", "wall with engravings").takeable(false),
            "engravings_cave",
        );

        world.add_object(
            Object::new("owners_manual", "ZORK owner's manual"),
            "studio",
        );

        world.add_object(Object::new("wrench", "wrench"), "maintenance_room");

        world.add_object(
            Object::new("control_panel", "control panel").takeable(false),
            "dam_room",
        );

        world.add_object(
            Object::new("nest", "bird's nest")
                .openable()
                .with_content("egg"),
            "up_a_tree",
        );

        world.add_object(
            Object::new("egg", "jewel-encrusted egg")
                .openable()
                .with_content("canary"),
            "nest",
        );

        world.add_object(Object::new("canary", "golden clockwork canary"), "egg");

        world.add_object(
            Object::new("broken_canary", "broken clockwork canary"),
            "broken_egg",
        );

        world
    }

    pub fn load_zork2() -> Self {
        let mut world = World::new();

        // ZORK2: 86 rooms, 145 objects

        world.add_room(
            Room::new("inside_barrow", "Inside the Barrow").with_exit("south", "narrow_tunnel"),
        );

        world.add_room(
            Room::new("narrow_tunnel", "Narrow Tunnel")
                .with_exit("north", "inside_barrow")
                .with_exit("south", "foot_bridge")
                .with_exit("cross", "foot_bridge"),
        );

        world.add_room(
            Room::new("foot_bridge", "Foot Bridge")
                .with_exit("north", "narrow_tunnel")
                .with_exit("south", "great_cavern"),
        );

        world.add_room(
            Room::new("great_cavern", "Great Cavern")
                .with_exit("ne", "foot_bridge")
                .with_exit("sw", "shallow_ford"),
        );

        world.add_room(
            Room::new("shallow_ford", "Shallow Ford")
                .with_exit("north", "great_cavern")
                .with_exit("south", "dark_tunnel")
                .with_exit("cross", "dark_tunnel"),
        );

        world.add_room(
            Room::new("dark_tunnel", "Dark Tunnel")
                .with_exit("ne", "shallow_ford")
                .with_exit("se", "garden_north")
                .with_exit("sw", "stream_path"),
        );

        world.add_room(
            Room::new("garden_north", "North End of Garden")
                .with_exit("in", "gazebo_room")
                .with_exit("north", "dark_tunnel")
                .with_exit("south", "formal_garden"),
        );

        world.add_room(Room::new("gazebo_room", "Gazebo").with_exit("out", "garden_north"));

        world.add_room(
            Room::new("formal_garden", "Formal Garden")
                .with_exit("west", "stream_path")
                .with_exit("north", "garden_north")
                .with_exit("south", "topiary_room"),
        );

        world.add_room(
            Room::new("topiary_room", "Topiary")
                .with_exit("west", "carousel_room")
                .with_exit("north", "formal_garden"),
        );

        world.add_room(
            Room::new("stream_path", "Path Near Stream")
                .with_exit("east", "formal_garden")
                .with_exit("ne", "dark_tunnel")
                .with_exit("sw", "carousel_room"),
        );

        world.add_room(
            Room::new("marble_hall", "Marble Hall")
                .with_exit("north", "deep_ford")
                .with_exit("south", "carousel_room"),
        );

        world.add_room(
            Room::new("deep_ford", "Deep Ford")
                .with_exit("north", "ravine_ledge")
                .with_exit("up", "ravine_ledge")
                .with_exit("south", "marble_hall"),
        );

        world.add_room(
            Room::new("ravine_ledge", "Ledge in Ravine")
                .with_exit("up", "tiny_room")
                .with_exit("south", "deep_ford")
                .with_exit("west", "ledge_tunnel")
                .with_exit("down", "deep_ford"),
        );

        world.add_room(
            Room::new("ledge_tunnel", "End of Ledge")
                .with_exit("east", "ravine_ledge")
                .with_exit("in", "dragon_room")
                .with_exit("north", "dragon_room"),
        );

        world.add_room(
            Room::new("dragon_room", "Dragon Room")
                .with_exit("east", "ledge_tunnel")
                .with_exit("west", "fresco_room")
                .with_exit("south", "stone_bridge")
                .with_exit("cross", "stone_bridge"),
        );

        world.add_room(
            Room::new("dragon_lair", "Dragon's Lair")
                .with_exit("south", "dragon_room")
                .with_exit("out", "dragon_room"),
        );

        world.add_room(
            Room::new("fresco_room", "Fresco Room")
                .with_exit("east", "dragon_room")
                .with_exit("west", "bank_entrance"),
        );

        world.add_room(
            Room::new("stone_bridge", "Stone Bridge")
                .with_exit("north", "dragon_room")
                .with_exit("south", "cool_room"),
        );

        world.add_room(
            Room::new("cool_room", "Cool Room")
                .with_exit("se", "carousel_room")
                .with_exit("north", "stone_bridge")
                .with_exit("west", "glacier_room")
                .with_exit("cross", "stone_bridge"),
        );

        world.add_room(
            Room::new("glacier_room", "Ice Room")
                .with_exit("east", "cool_room")
                .with_exit("up", "lava_tube"),
        );

        world.add_room(
            Room::new("lava_tube", "Lava Tube")
                .with_exit("down", "glacier_room")
                .with_exit("up", "volcano_view")
                .with_exit("south", "cobwebby_corridor"),
        );

        world.add_room(
            Room::new("cobwebby_corridor", "Cobwebby Corridor")
                .with_exit("in", "lava_tube")
                .with_exit("up", "lava_tube")
                .with_exit("north", "lava_tube")
                .with_exit("ne", "carousel_room")
                .with_exit("sw", "guardian_room")
                .with_exit("down", "guardian_room"),
        );

        world.add_room(Room::new("room_8", "Room 8").with_exit("east", "carousel_room"));

        world.add_room(
            Room::new("menhir_room", "Menhir Room")
                .with_exit("north", "carousel_room")
                .with_exit("south", "stairway_top"),
        );

        world.add_room(Room::new("kennel", "Kennel"));

        world.add_room(
            Room::new("stairway_top", "Stairway")
                .with_exit("down", "diamond_5")
                .with_exit("north", "menhir_room"),
        );

        world.add_room(Room::new("diamond_1", "Oddly-angled Room").with_exit("se", "diamond_5"));

        world.add_room(
            Room::new("diamond_2", "Oddly-angled Room")
                .with_exit("south", "diamond_5")
                .with_exit("se", "diamond_6")
                .with_exit("sw", "diamond_4"),
        );

        world.add_room(Room::new("diamond_3", "Oddly-angled Room").with_exit("sw", "diamond_5"));

        world.add_room(
            Room::new("diamond_4", "Oddly-angled Room")
                .with_exit("ne", "diamond_2")
                .with_exit("se", "diamond_8")
                .with_exit("east", "diamond_5"),
        );

        world.add_room(
            Room::new("diamond_5", "Oddly-angled Room")
                .with_exit("nw", "diamond_1")
                .with_exit("north", "diamond_2")
                .with_exit("ne", "diamond_3")
                .with_exit("west", "diamond_4")
                .with_exit("east", "diamond_6")
                .with_exit("sw", "diamond_7")
                .with_exit("south", "diamond_8")
                .with_exit("se", "diamond_9")
                .with_exit("down", "cerberus_room")
                .with_exit("up", "stairway_top"),
        );

        world.add_room(
            Room::new("diamond_6", "Oddly-angled Room")
                .with_exit("west", "diamond_5")
                .with_exit("nw", "diamond_2")
                .with_exit("sw", "diamond_8"),
        );

        world.add_room(Room::new("diamond_7", "Oddly-angled Room").with_exit("ne", "diamond_5"));

        world.add_room(
            Room::new("diamond_8", "Oddly-angled Room")
                .with_exit("north", "diamond_5")
                .with_exit("nw", "diamond_4")
                .with_exit("ne", "diamond_6"),
        );

        world.add_room(Room::new("diamond_9", "Oddly-angled Room").with_exit("nw", "diamond_5"));

        world.add_room(Room::new("cerberus_room", "Cerberus Room").with_exit("up", "diamond_5"));

        world.add_room(
            Room::new("crypt_anteroom", "Crypt Anteroom").with_exit("west", "cerberus_room"),
        );

        world.add_room(Room::new("crypt_room", "Crypt"));

        world.add_room(Room::new("zork3", "Landing"));

        world.add_room(
            Room::new("guardian_room", "Guarded Room").with_exit("north", "cobwebby_corridor"),
        );

        world.add_room(
            Room::new("wizards_workshop", "Wizard's Workshop")
                .with_exit("west", "workbench_room")
                .with_exit("south", "trophy_room"),
        );

        world.add_room(
            Room::new("workbench_room", "Wizard's Workroom")
                .with_exit("east", "wizards_workshop")
                .with_exit("south", "pentagram_room")
                .with_exit("west", "aquarium_room"),
        );

        world.add_room(
            Room::new("trophy_room", "Trophy Room").with_exit("north", "wizards_workshop"),
        );

        world.add_room(
            Room::new("pentagram_room", "Pentagram Room").with_exit("north", "workbench_room"),
        );

        world.add_room(
            Room::new("aquarium_room", "Aquarium Room")
                .with_exit("east", "workbench_room")
                .with_exit("in", "in_aquarium")
                .with_exit("south", "wizards_quarters"),
        );

        world.add_room(Room::new("in_aquarium", "Murky Room").with_exit("out", "aquarium_room"));

        world.add_room(
            Room::new("wizards_quarters", "Wizard's Quarters").with_exit("north", "aquarium_room"),
        );

        world.add_room(
            Room::new("carousel_room", "Carousel Room")
                .with_exit("north", "marble_hall")
                .with_exit("ne", "stream_path")
                .with_exit("east", "topiary_room")
                .with_exit("se", "riddle_room")
                .with_exit("south", "menhir_room")
                .with_exit("sw", "cobwebby_corridor")
                .with_exit("west", "room_8")
                .with_exit("nw", "cool_room"),
        );

        world.add_room(
            Room::new("riddle_room", "Riddle Room")
                .with_exit("down", "carousel_room")
                .with_exit("nw", "carousel_room"),
        );

        world.add_room(
            Room::new("pearl_room", "Pearl Room")
                .with_exit("east", "well_bottom")
                .with_exit("west", "riddle_room"),
        );

        world.add_room(
            Room::new("volcano_bottom", "Volcano Bottom").with_exit("north", "lava_room"),
        );

        world.add_room(Room::new("vair_1", "Volcano Core"));

        world.add_room(
            Room::new("vair_2", "Volcano Near Small Ledge")
                .with_exit("west", "ledge_1")
                .with_exit("land", "ledge_1"),
        );

        world.add_room(Room::new("vair_3", "Volcano by Viewing Ledge"));

        world.add_room(
            Room::new("vair_4", "Volcano Near Wide Ledge")
                .with_exit("land", "ledge_2")
                .with_exit("west", "ledge_2"),
        );

        world.add_room(Room::new("ledge_1", "Narrow Ledge").with_exit("south", "library"));

        world.add_room(
            Room::new("library", "Library")
                .with_exit("north", "ledge_1")
                .with_exit("out", "ledge_1"),
        );

        world.add_room(Room::new("volcano_view", "Volcano View").with_exit("east", "lava_tube"));

        world.add_room(Room::new("ledge_2", "Wide Ledge").with_exit("south", "safe_room"));

        world.add_room(Room::new("safe_room", "Dusty Room").with_exit("north", "ledge_2"));

        world.add_room(
            Room::new("lava_room", "Lava Room")
                .with_exit("south", "volcano_bottom")
                .with_exit("east", "glacier_room"),
        );

        world.add_room(Room::new("magnet_room", "Low Room"));

        world.add_room(
            Room::new("machine_room", "Machine Room")
                .with_exit("west", "magnet_room")
                .with_exit("south", "cage_room"),
        );

        world.add_room(
            Room::new("cage_room", "Dingy Closet")
                .with_exit("out", "machine_room")
                .with_exit("north", "machine_room"),
        );

        world.add_room(Room::new("in_cage", "Cage"));

        world.add_room(Room::new("well_top", "Top of Well").with_exit("east", "tea_room"));

        world.add_room(Room::new("well_bottom", "Circular Room").with_exit("west", "pearl_room"));

        world.add_room(
            Room::new("tea_room", "Tea Room")
                .with_exit("west", "well_top")
                .with_exit("nw", "magnet_room"),
        );

        world.add_room(Room::new("posts_room", "Posts Room").with_exit("east", "pool_room"));

        world.add_room(
            Room::new("pool_room", "Pool Room")
                .with_exit("out", "posts_room")
                .with_exit("west", "posts_room"),
        );

        world.add_room(
            Room::new("bank_entrance", "Bank Entrance")
                .with_exit("nw", "teller_west")
                .with_exit("ne", "teller_east")
                .with_exit("east", "fresco_room"),
        );

        world.add_room(
            Room::new("teller_west", "West Teller's Room")
                .with_exit("north", "viewing_west")
                .with_exit("south", "bank_entrance")
                .with_exit("west", "depository"),
        );

        world.add_room(
            Room::new("teller_east", "East Teller's Room")
                .with_exit("north", "viewing_east")
                .with_exit("south", "bank_entrance")
                .with_exit("east", "depository"),
        );

        world.add_room(
            Room::new("viewing_west", "West Viewing Room").with_exit("south", "bank_entrance"),
        );

        world.add_room(
            Room::new("viewing_east", "East Viewing Room").with_exit("south", "bank_entrance"),
        );

        world.add_room(Room::new("small_room", "Small Room"));

        world.add_room(Room::new("vault", "Vault"));

        world.add_room(Room::new("depository", "Safety Depository").with_exit("south", "office"));

        world.add_room(Room::new("office", "Chairman's Office").with_exit("north", "depository"));

        world.add_room(Room::new("dreary_room", "Dreary Room"));

        world.add_room(Room::new("tiny_room", "Tiny Room").with_exit("down", "ravine_ledge"));

        world.add_room(
            Room::new("dead_palantir_1", "Room of Red Mist").with_exit("west", "dead_palantir_2"),
        );

        world.add_room(
            Room::new("dead_palantir_2", "Room of Blue Mist").with_exit("west", "dead_palantir_3"),
        );

        world.add_room(
            Room::new("dead_palantir_3", "Room of White Mist").with_exit("west", "dead_palantir_4"),
        );

        world.add_room(Room::new("dead_palantir_4", "Room of Black Mist"));

        world.add_object(
            Object::new("hedges", "hedge").takeable(false),
            "topiary_room",
        );

        world.add_object(
            Object::new("chest", "rotten wooden chest")
                .openable()
                .with_content("statuette"),
            "dragon_lair",
        );

        world.add_object(
            Object::new("dragon", "huge red dragon").takeable(false),
            "dragon_room",
        );

        world.add_object(
            Object::new("princess", "beautiful princess").takeable(false),
            "dragon_lair",
        );

        world.add_object(Object::new("collar", "gigantic dog collar"), "kennel");

        world.add_object(
            Object::new("cerberus", "three-headed dog")
                .takeable(false)
                .openable(),
            "cerberus_room",
        );

        world.add_object(
            Object::new("heads", "set of poled heads").takeable(false),
            "crypt_room",
        );

        world.add_object(
            Object::new("crypt", "marble crypt").takeable(false),
            "crypt_room",
        );

        world.add_object(
            Object::new("door_keeper", "lizard").takeable(false),
            "guardian_room",
        );

        world.add_object(
            Object::new("arcana", "arcane item").takeable(false),
            "workbench_room",
        );

        world.add_object(
            Object::new("workbench", "Wizard's workbench")
                .takeable(false)
                .openable(),
            "workbench_room",
        );

        world.add_object(
            Object::new("stand_1", "ruby stand")
                .takeable(false)
                .openable(),
            "workbench",
        );

        world.add_object(
            Object::new("stand_2", "sapphire stand")
                .takeable(false)
                .openable(),
            "workbench",
        );

        world.add_object(
            Object::new("stand_3", "diamond stand")
                .takeable(false)
                .openable(),
            "workbench",
        );

        world.add_object(
            Object::new("degree", "degree").takeable(false),
            "trophy_room",
        );

        world.add_object(
            Object::new("wands", "set of used wands").takeable(false),
            "trophy_room",
        );

        world.add_object(
            Object::new("trophy_sword", "nicked swords").takeable(false),
            "trophy_room",
        );

        world.add_object(
            Object::new("trophy_bottles", "small bottles").takeable(false),
            "trophy_room",
        );

        world.add_object(
            Object::new("warning_label", "warning label").takeable(false),
            "trophy_room",
        );

        world.add_object(
            Object::new("wizard_case", "Wizard's trophy cabinet")
                .takeable(false)
                .openable(),
            "trophy_room",
        );

        world.add_object(
            Object::new("pentagram", "pentagram")
                .takeable(false)
                .openable(),
            "pentagram_room",
        );

        world.add_object(
            Object::new("aquarium", "aquarium")
                .takeable(false)
                .openable(),
            "aquarium_room",
        );

        world.add_object(
            Object::new("palantir_3", "clear crystal sphere"),
            "in_aquarium",
        );

        world.add_object(
            Object::new("serpent", "baby sea serpent").takeable(false),
            "aquarium",
        );

        world.add_object(
            Object::new("riddle_door", "stone door")
                .takeable(false)
                .openable(),
            "riddle_room",
        );

        world.add_object(Object::new("match", "matchbook"), "gazebo_table");

        world.add_object(
            Object::new("balloon", "basket").takeable(false).openable(),
            "volcano_bottom",
        );

        world.add_object(
            Object::new("safe", "box")
                .takeable(false)
                .openable()
                .with_content("card")
                .with_content("crown"),
            "safe_room",
        );

        world.add_object(
            Object::new("braided_wire", "braided wire").takeable(false),
            "balloon",
        );

        world.add_object(Object::new("brick", "brick").openable(), "marble_hall");

        world.add_object(
            Object::new("round_button", "round button").takeable(false),
            "machine_room",
        );

        world.add_object(
            Object::new("square_button", "square button").takeable(false),
            "machine_room",
        );

        world.add_object(
            Object::new("triangular_button", "triangular button").takeable(false),
            "machine_room",
        );

        world.add_object(
            Object::new("cloth_bag", "cloth bag").takeable(false),
            "balloon",
        );

        world.add_object(Object::new("violin", "fancy violin"), "iron_box");

        world.add_object(
            Object::new("ice", "glacier").takeable(false),
            "glacier_room",
        );

        world.add_object(
            Object::new("flask", "stoppered glass flask filled with liquid"),
            "pool_room",
        );

        world.add_object(
            Object::new("robot_label", "green piece of paper"),
            "magnet_room",
        );

        world.add_object(
            Object::new("slot", "hole").takeable(false).openable(),
            "safe_room",
        );

        world.add_object(Object::new("lamp", "lamp"), "inside_barrow");

        world.add_object(
            Object::new("alice_table", "large oblong table")
                .takeable(false)
                .openable(),
            "tea_room",
        );

        world.add_object(Object::new("pearl", "pearl necklace"), "pearl_room");

        world.add_object(
            Object::new("eat_me_cake", "cake frosted with green letters"),
            "alice_table",
        );

        world.add_object(
            Object::new("blue_icing", "cake frosted with blue letters"),
            "alice_table",
        );

        world.add_object(
            Object::new("orange_icing", "cake frosted with orange letters"),
            "alice_table",
        );

        world.add_object(
            Object::new("red_icing", "cake frosted with red letters"),
            "alice_table",
        );

        world.add_object(Object::new("leak", "leak").takeable(false), "pool_room");

        world.add_object(
            Object::new("pool", "pool of tears").takeable(false),
            "pool_room",
        );

        world.add_object(
            Object::new("green_book", "green book").openable(),
            "library",
        );

        world.add_object(Object::new("blue_book", "blue book").openable(), "library");

        world.add_object(
            Object::new("white_book", "white book").openable(),
            "library",
        );

        world.add_object(
            Object::new("purple_book", "purple book")
                .openable()
                .with_content("stamp"),
            "library",
        );

        world.add_object(
            Object::new("receptacle", "receptacle")
                .takeable(false)
                .openable(),
            "balloon",
        );

        world.add_object(
            Object::new("robot", "robot").takeable(false).openable(),
            "magnet_room",
        );

        world.add_object(Object::new("ruby", "ruby"), "lava_room");

        world.add_object(
            Object::new("iron_box", "steel box").openable(),
            "carousel_room",
        );

        world.add_object(
            Object::new("cage", "solid steel cage").takeable(false),
            "cage_room",
        );

        world.add_object(Object::new("candy", "package of candy"), "pool_room");

        world.add_object(
            Object::new("top_etchings", "wall with etchings").takeable(false),
            "well_top",
        );

        world.add_object(
            Object::new("bottom_etchings", "wall with etchings").takeable(false),
            "well_bottom",
        );

        world.add_object(Object::new("fuse", "black string"), "cobwebby_corridor");

        world.add_object(
            Object::new("bucket", "wooden bucket")
                .takeable(false)
                .openable(),
            "well_bottom",
        );

        world.add_object(
            Object::new("posts", "group of wooden posts").takeable(false),
            "posts_room",
        );

        world.add_object(Object::new("bills", "stack of zorkmid bills"), "vault");

        world.add_object(
            Object::new("portrait", "portrait of J. Pierpont Flathead"),
            "office",
        );

        world.add_object(Object::new("bank_brochure", "bank brochure"), "depository");

        world.add_object(
            Object::new("cube", "large stone cube").takeable(false),
            "depository",
        );

        world.add_object(
            Object::new("curtain", "shimmering curtain of light").takeable(false),
            "depository",
        );

        world.add_object(
            Object::new("deposit_box", "safety deposit box"),
            "gnome_of_zurich",
        );

        world.add_object(
            Object::new("lid_1", "metal lid").takeable(false).openable(),
            "tiny_room",
        );

        world.add_object(
            Object::new("lid_2", "metal lid").takeable(false).openable(),
            "dreary_room",
        );

        world.add_object(
            Object::new("ptable", "table").takeable(false).openable(),
            "dreary_room",
        );

        world.add_object(
            Object::new("pcrack", "narrow crack").takeable(false),
            "dreary_room",
        );

        world.add_object(
            Object::new("keyhole_1", "keyhole")
                .takeable(false)
                .openable(),
            "tiny_room",
        );

        world.add_object(
            Object::new("keyhole_2", "keyhole")
                .takeable(false)
                .openable(),
            "dreary_room",
        );

        world.add_object(
            Object::new("letter_opener", "letter opener"),
            "gazebo_table",
        );

        world.add_object(
            Object::new("palantir_2", "blue crystal sphere"),
            "dreary_room",
        );

        world.add_object(
            Object::new("gazebo_table", "table")
                .takeable(false)
                .openable(),
            "gazebo_room",
        );

        world.add_object(Object::new("newspaper", "newspaper"), "gazebo_table");

        world.add_object(
            Object::new("place_mat", "place mat").openable(),
            "gazebo_table",
        );

        world.add_object(
            Object::new("teapot", "china teapot").openable(),
            "gazebo_table",
        );

        world.add_object(Object::new("gold_key", "delicate gold key"), "unicorn");

        world.add_object(Object::new("ribbon", "ribbon").takeable(false), "unicorn");

        world.add_object(Object::new("wand", "Wizard's magic wand"), "wizard");

        world.add_object(Object::new("sword", "elvish sword"), "inside_barrow");

        world
    }

    pub fn load_zork3() -> Self {
        let mut world = World::new();

        // ZORK3: 35 rooms, 44 objects

        world.add_room(
            Room::new("cp_ante", "Royal Puzzle Entrance")
                .with_exit("west", "cp_out")
                .with_exit("north", "museum_entrance")
                .with_exit("up", "museum_entrance"),
        );

        world.add_room(
            Room::new("cp_out", "Side Room")
                .with_exit("north", "cp_ante")
                .with_exit("up", "cp_ante"),
        );

        world.add_room(Room::new("cp", "Room in a Puzzle"));

        world.add_room(
            Room::new("mrd", "Hallway")
                .with_exit("north", "front_door")
                .with_exit("ne", "front_door")
                .with_exit("nw", "front_door"),
        );

        world.add_room(Room::new("mrg", "Hallway"));

        world.add_room(Room::new("mrc", "Hallway"));

        world.add_room(Room::new("mrb", "Hallway"));

        world.add_room(Room::new("mra", "Hallway").with_exit("south", "mreye"));

        world.add_room(
            Room::new("mrde", "Narrow Room")
                .with_exit("north", "front_door")
                .with_exit("south", "mrg"),
        );

        world.add_room(
            Room::new("mrdw", "Narrow Room")
                .with_exit("north", "front_door")
                .with_exit("south", "mrg"),
        );

        world.add_room(
            Room::new("mrge", "Narrow Room")
                .with_exit("north", "mrd")
                .with_exit("south", "mrc"),
        );

        world.add_room(
            Room::new("mrgw", "Narrow Room")
                .with_exit("north", "mrd")
                .with_exit("south", "mrc"),
        );

        world.add_room(
            Room::new("mrce", "Narrow Room")
                .with_exit("north", "mrg")
                .with_exit("south", "mrb"),
        );

        world.add_room(
            Room::new("mrcw", "Narrow Room")
                .with_exit("north", "mrg")
                .with_exit("south", "mrb"),
        );

        world.add_room(
            Room::new("mrbe", "Narrow Room")
                .with_exit("north", "mrc")
                .with_exit("south", "mra"),
        );

        world.add_room(
            Room::new("mrbw", "Narrow Room")
                .with_exit("north", "mrc")
                .with_exit("south", "mra"),
        );

        world.add_room(
            Room::new("mrae", "Narrow Room")
                .with_exit("north", "mrb")
                .with_exit("south", "mreye"),
        );

        world.add_room(
            Room::new("mraw", "Narrow Room")
                .with_exit("north", "mrb")
                .with_exit("south", "mreye"),
        );

        world.add_room(Room::new("in_mirror", "Inside Mirror"));

        world.add_room(Room::new("mr_ante", "Button Room").with_exit("north", "mreye"));

        world.add_room(Room::new("mreye", "Beam Room").with_exit("south", "mr_ante"));

        world.add_room(
            Room::new("mstairs", "Engravings Room")
                .with_exit("sw", "damp_passage")
                .with_exit("se", "dead_end"),
        );

        world.add_room(
            Room::new("dead_end", "Dead End")
                .with_exit("west", "damp_passage")
                .with_exit("nw", "mstairs"),
        );

        world.add_room(
            Room::new("damp_passage", "Damp Passage")
                .with_exit("west", "junction")
                .with_exit("east", "dead_end")
                .with_exit("ne", "mstairs"),
        );

        world.add_room(
            Room::new("east_corridor", "East Corridor")
                .with_exit("north", "north_corridor")
                .with_exit("south", "south_corridor"),
        );

        world.add_room(
            Room::new("west_corridor", "West Corridor")
                .with_exit("north", "north_corridor")
                .with_exit("south", "south_corridor"),
        );

        world.add_room(
            Room::new("south_corridor", "South Corridor")
                .with_exit("east", "east_corridor")
                .with_exit("west", "west_corridor")
                .with_exit("south", "behind_door"),
        );

        world.add_room(
            Room::new("behind_door", "Narrow Corridor").with_exit("north", "south_corridor"),
        );

        world.add_room(
            Room::new("front_door", "Dungeon Entrance")
                .with_exit("se", "mrde")
                .with_exit("sw", "mrdw"),
        );

        world.add_room(
            Room::new("north_corridor", "North Corridor")
                .with_exit("east", "east_corridor")
                .with_exit("west", "west_corridor")
                .with_exit("north", "parapet"),
        );

        world.add_room(Room::new("parapet", "Parapet").with_exit("south", "north_corridor"));

        world.add_room(Room::new("cell", "Prison Cell"));

        world.add_room(Room::new("prison_cell", "Prison Cell"));

        world.add_room(Room::new("good_cell", "Prison Cell"));

        world.add_room(Room::new("nirvana", "Treasury of Zork"));

        world.add_object(Object::new("warning_note", "warning note"), "cp_ante");

        world.add_object(
            Object::new("dungeon_master", "dungeon master").takeable(false),
            "behind_door",
        );

        world.add_object(Object::new("runes", "runes").takeable(false), "mstairs");

        world.add_object(Object::new("t_bar", "T-bar").takeable(false), "in_mirror");

        world.add_object(
            Object::new("black_panel", "black panel").takeable(false),
            "in_mirror",
        );

        world.add_object(
            Object::new("compass_arrow", "compass arrow").takeable(false),
            "in_mirror",
        );

        world.add_object(
            Object::new("dial_button", "large button").takeable(false),
            "parapet",
        );

        world.add_object(
            Object::new("long_pole", "long pole").takeable(false),
            "in_mirror",
        );

        world.add_object(
            Object::new("mahogany_panel", "mahogany panel").takeable(false),
            "in_mirror",
        );

        world.add_object(
            Object::new("pine_panel", "pine panel").takeable(false),
            "in_mirror",
        );

        world.add_object(
            Object::new("beam", "red beam of light")
                .takeable(false)
                .openable(),
            "mreye",
        );

        world.add_object(
            Object::new("red_button", "red button").takeable(false),
            "mr_ante",
        );

        world.add_object(
            Object::new("red_panel", "red panel").takeable(false),
            "in_mirror",
        );

        world.add_object(
            Object::new("short_pole", "short pole").takeable(false),
            "in_mirror",
        );

        world.add_object(Object::new("sundial", "sundial").takeable(false), "parapet");

        world.add_object(
            Object::new("yellow_panel", "yellow panel").takeable(false),
            "in_mirror",
        );

        world.add_object(
            Object::new("white_panel", "white panel").takeable(false),
            "in_mirror",
        );

        world.add_object(
            Object::new("wooden_bar", "wooden bar").takeable(false),
            "in_mirror",
        );

        world.add_object(Object::new("cp_hole", "hole").takeable(false), "cp_ante");

        world
    }
}
