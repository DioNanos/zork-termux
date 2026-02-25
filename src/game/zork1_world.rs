// ZORK1 - 68 rooms, 121 objects

world.add_room(Room::new("west_of_house", "West of House")
    .with_exit("north", "north_of_house")
    .with_exit("south", "south_of_house")
    .with_exit("ne", "north_of_house")
    .with_exit("se", "south_of_house")
    .with_exit("west", "forest_1")
);

world.add_room(Room::new("stone_barrow", "Stone Barrow")
    .with_exit("ne", "west_of_house")
);

world.add_room(Room::new("north_of_house", "North of House")
    .with_exit("sw", "west_of_house")
    .with_exit("se", "east_of_house")
    .with_exit("west", "west_of_house")
    .with_exit("east", "east_of_house")
    .with_exit("north", "path")
);

world.add_room(Room::new("south_of_house", "South of House")
    .with_exit("west", "west_of_house")
    .with_exit("east", "east_of_house")
    .with_exit("ne", "east_of_house")
    .with_exit("nw", "west_of_house")
    .with_exit("south", "forest_3")
);

world.add_room(Room::new("east_of_house", "Behind House")
    .with_exit("north", "north_of_house")
    .with_exit("south", "south_of_house")
    .with_exit("sw", "south_of_house")
    .with_exit("nw", "north_of_house")
    .with_exit("east", "clearing")
);

world.add_room(Room::new("forest_1", "Forest")
    .with_exit("north", "grating_clearing")
    .with_exit("east", "path")
    .with_exit("south", "forest_3")
);

world.add_room(Room::new("forest_2", "Forest")
    .with_exit("east", "mountains")
    .with_exit("south", "clearing")
    .with_exit("west", "path")
);

world.add_room(Room::new("mountains", "Forest")
    .with_exit("north", "forest_2")
    .with_exit("south", "forest_2")
    .with_exit("west", "forest_2")
);

world.add_room(Room::new("forest_3", "Forest")
    .with_exit("north", "clearing")
    .with_exit("west", "forest_1")
    .with_exit("nw", "south_of_house")
);

world.add_room(Room::new("path", "Forest Path")
    .with_exit("up", "up_a_tree")
    .with_exit("north", "grating_clearing")
    .with_exit("east", "forest_2")
    .with_exit("south", "north_of_house")
    .with_exit("west", "forest_1")
);

world.add_room(Room::new("up_a_tree", "Up a Tree")
    .with_exit("down", "path")
);

world.add_room(Room::new("grating_clearing", "Clearing")
    .with_exit("east", "forest_2")
    .with_exit("west", "forest_1")
    .with_exit("south", "path")
);

world.add_room(Room::new("clearing", "Clearing")
    .with_exit("east", "canyon_view")
    .with_exit("north", "forest_2")
    .with_exit("south", "forest_3")
    .with_exit("west", "east_of_house")
);

world.add_room(Room::new("kitchen", "Kitchen")
    .with_exit("west", "living_room")
    .with_exit("up", "attic")
);

world.add_room(Room::new("attic", "Attic")
    .with_exit("down", "kitchen")
);

world.add_room(Room::new("living_room", "Living Room")
    .with_exit("east", "kitchen")
);

world.add_room(Room::new("cellar", "Cellar")
    .with_exit("north", "troll_room")
    .with_exit("south", "east_of_chasm")
);

world.add_room(Room::new("troll_room", "The Troll Room")
    .with_exit("south", "cellar")
);

world.add_room(Room::new("east_of_chasm", "East of Chasm")
    .with_exit("north", "cellar")
    .with_exit("east", "gallery")
);

world.add_room(Room::new("gallery", "Gallery")
    .with_exit("west", "east_of_chasm")
    .with_exit("north", "studio")
);

world.add_room(Room::new("studio", "Studio")
    .with_exit("south", "gallery")
);

world.add_room(Room::new("maze_1", "Maze")
    .with_exit("east", "troll_room")
    .with_exit("north", "maze_1")
    .with_exit("south", "maze_2")
    .with_exit("west", "maze_4")
);

world.add_room(Room::new("maze_2", "Maze")
    .with_exit("south", "maze_1")
    .with_exit("east", "maze_3")
);

world.add_room(Room::new("maze_3", "Maze")
    .with_exit("west", "maze_2")
    .with_exit("north", "maze_4")
    .with_exit("up", "maze_5")
);

world.add_room(Room::new("maze_4", "Maze")
    .with_exit("west", "maze_3")
    .with_exit("north", "maze_1")
    .with_exit("east", "dead_end_1")
);

world.add_room(Room::new("dead_end_1", "Dead End")
    .with_exit("south", "maze_4")
);

world.add_room(Room::new("maze_5", "Maze")
    .with_exit("east", "dead_end_2")
    .with_exit("north", "maze_3")
    .with_exit("sw", "maze_6")
);

world.add_room(Room::new("dead_end_2", "Dead End")
    .with_exit("west", "maze_5")
);

world.add_room(Room::new("maze_6", "Maze")
    .with_exit("down", "maze_5")
    .with_exit("east", "maze_7")
    .with_exit("west", "maze_6")
    .with_exit("up", "maze_9")
);

world.add_room(Room::new("maze_7", "Maze")
    .with_exit("up", "maze_14")
    .with_exit("west", "maze_6")
    .with_exit("east", "maze_8")
    .with_exit("south", "maze_15")
);

world.add_room(Room::new("maze_8", "Maze")
    .with_exit("ne", "maze_7")
    .with_exit("west", "maze_8")
    .with_exit("se", "dead_end_3")
);

world.add_room(Room::new("dead_end_3", "Dead End")
    .with_exit("north", "maze_8")
);

world.add_room(Room::new("maze_9", "Maze")
    .with_exit("north", "maze_6")
    .with_exit("east", "maze_10")
    .with_exit("south", "maze_13")
    .with_exit("west", "maze_12")
    .with_exit("nw", "maze_9")
);

world.add_room(Room::new("maze_10", "Maze")
    .with_exit("east", "maze_9")
    .with_exit("west", "maze_13")
    .with_exit("up", "maze_11")
);

world.add_room(Room::new("maze_11", "Maze")
    .with_exit("ne", "grating_room")
    .with_exit("down", "maze_10")
    .with_exit("nw", "maze_13")
    .with_exit("sw", "maze_12")
);

world.add_room(Room::new("grating_room", "Grating Room")
    .with_exit("sw", "maze_11")
);

world.add_room(Room::new("maze_12", "Maze")
    .with_exit("sw", "maze_11")
    .with_exit("east", "maze_13")
    .with_exit("up", "maze_9")
    .with_exit("north", "dead_end_4")
);

world.add_room(Room::new("dead_end_4", "Dead End")
    .with_exit("south", "maze_12")
);

world.add_room(Room::new("maze_13", "Maze")
    .with_exit("east", "maze_9")
    .with_exit("down", "maze_12")
    .with_exit("south", "maze_10")
    .with_exit("west", "maze_11")
);

world.add_room(Room::new("maze_14", "Maze")
    .with_exit("west", "maze_15")
    .with_exit("nw", "maze_14")
    .with_exit("ne", "maze_7")
    .with_exit("south", "maze_7")
);

world.add_room(Room::new("maze_15", "Maze")
    .with_exit("west", "maze_14")
    .with_exit("south", "maze_7")
    .with_exit("se", "cyclops_room")
);

world.add_room(Room::new("cyclops_room", "Cyclops Room")
    .with_exit("nw", "maze_15")
);

world.add_room(Room::new("strange_passage", "Strange Passage")
    .with_exit("west", "cyclops_room")
    .with_exit("in", "cyclops_room")
    .with_exit("east", "living_room")
);

world.add_room(Room::new("treasure_room", "Treasure Room")
    .with_exit("down", "cyclops_room")
);

world.add_room(Room::new("reservoir_south", "Reservoir South")
    .with_exit("se", "deep_canyon")
    .with_exit("sw", "chasm_room")
    .with_exit("east", "dam_room")
    .with_exit("west", "stream_view")
);

world.add_room(Room::new("reservoir", "Reservoir")
    .with_exit("north", "reservoir_north")
    .with_exit("south", "reservoir_south")
    .with_exit("up", "in_stream")
    .with_exit("west", "in_stream")
);

world.add_room(Room::new("reservoir_north", "Reservoir North")
    .with_exit("north", "atlantis_room")
);

world.add_room(Room::new("stream_view", "Stream View")
    .with_exit("east", "reservoir_south")
);

world.add_room(Room::new("in_stream", "Stream")
    .with_exit("land", "stream_view")
    .with_exit("down", "reservoir")
    .with_exit("east", "reservoir")
);

world.add_room(Room::new("mirror_room_1", "Mirror Room")
    .with_exit("north", "cold_passage")
    .with_exit("west", "twisting_passage")
    .with_exit("east", "small_cave")
);

world.add_room(Room::new("mirror_room_2", "Mirror Room")
    .with_exit("west", "winding_passage")
    .with_exit("north", "narrow_passage")
    .with_exit("east", "tiny_cave")
);

world.add_room(Room::new("small_cave", "Cave")
    .with_exit("north", "mirror_room_1")
    .with_exit("down", "atlantis_room")
    .with_exit("south", "atlantis_room")
    .with_exit("west", "twisting_passage")
);

world.add_room(Room::new("tiny_cave", "Cave")
    .with_exit("north", "mirror_room_2")
    .with_exit("west", "winding_passage")
    .with_exit("down", "entrance_to_hades")
);

world.add_room(Room::new("cold_passage", "Cold Passage")
    .with_exit("south", "mirror_room_1")
    .with_exit("west", "slide_room")
);

world.add_room(Room::new("narrow_passage", "Narrow Passage")
    .with_exit("north", "round_room")
    .with_exit("south", "mirror_room_2")
);

world.add_room(Room::new("winding_passage", "Winding Passage")
    .with_exit("north", "mirror_room_2")
    .with_exit("east", "tiny_cave")
);

world.add_room(Room::new("twisting_passage", "Twisting Passage")
    .with_exit("north", "mirror_room_1")
    .with_exit("east", "small_cave")
);

world.add_room(Room::new("atlantis_room", "Atlantis Room")
    .with_exit("up", "small_cave")
    .with_exit("south", "reservoir_north")
);

world.add_room(Room::new("ew_passage", "East-West Passage")
    .with_exit("east", "round_room")
    .with_exit("west", "troll_room")
    .with_exit("down", "chasm_room")
    .with_exit("north", "chasm_room")
);

world.add_room(Room::new("round_room", "Round Room")
    .with_exit("east", "loud_room")
    .with_exit("west", "ew_passage")
    .with_exit("north", "ns_passage")
    .with_exit("south", "narrow_passage")
    .with_exit("se", "engravings_cave")
);

world.add_room(Room::new("deep_canyon", "Deep Canyon")
    .with_exit("nw", "reservoir_south")
    .with_exit("east", "dam_room")
    .with_exit("sw", "ns_passage")
    .with_exit("down", "loud_room")
);

world.add_room(Room::new("damp_cave", "Damp Cave")
    .with_exit("west", "loud_room")
    .with_exit("east", "white_cliffs_north")
);

world.add_room(Room::new("loud_room", "Loud Room")
    .with_exit("east", "damp_cave")
    .with_exit("west", "round_room")
    .with_exit("up", "deep_canyon")
);

world.add_room(Room::new("ns_passage", "North-South Passage")
    .with_exit("north", "chasm_room")
    .with_exit("ne", "deep_canyon")
    .with_exit("south", "round_room")
);

world.add_room(Room::new("chasm_room", "Chasm")
    .with_exit("ne", "reservoir_south")
    .with_exit("sw", "ew_passage")
    .with_exit("up", "ew_passage")
    .with_exit("south", "ns_passage")
);

world.add_room(Room::new("entrance_to_hades", "Entrance to Hades")
    .with_exit("up", "tiny_cave")
);

world.add_room(Room::new("land_of_living_dead", "Land of the Dead")
    .with_exit("out", "entrance_to_hades")
    .with_exit("north", "entrance_to_hades")
);

world.add_room(Room::new("torch_room", "Torch Room")
    .with_exit("south", "north_temple")
    .with_exit("down", "north_temple")
);

world.add_object(
    Object::new("mountain_range", "mountain range")
        .takeable(false)
    , "mountains"
);

world.add_object(
    Object::new("water", "quantity of water")
    , "bottle"
);

world.add_object(
    Object::new("ghosts", "number of ghosts")
        .takeable(false)
    , "entrance_to_hades"
);

world.add_object(
    Object::new("skull", "crystal skull")
    , "land_of_living_dead"
);

world.add_object(
    Object::new("lowered_basket", "basket")
        .takeable(false)
    , "lower_shaft"
);

world.add_object(
    Object::new("raised_basket", "basket")
        .takeable(false)
        .openable()
    , "shaft_room"
);

world.add_object(
    Object::new("lunch", "lunch")
    , "sandwich_bag"
);

world.add_object(
    Object::new("bat", "bat")
        .takeable(false)
    , "bat_room"
);

world.add_object(
    Object::new("bell", "brass bell")
    , "north_temple"
);

world.add_object(
    Object::new("axe", "bloody axe")
    , "troll"
);

world.add_object(
    Object::new("bolt", "bolt")
        .takeable(false)
    , "dam_room"
);

world.add_object(
    Object::new("bubble", "green bubble")
        .takeable(false)
    , "dam_room"
);

world.add_object(
    Object::new("altar", "altar")
        .takeable(false)
        .openable()
    , "south_temple"
);

world.add_object(
    Object::new("book", "black book")
        .openable()
    , "altar"
);

world.add_object(
    Object::new("sceptre", "sceptre")
    , "coffin"
);

world.add_object(
    Object::new("timbers", "broken timber")
    , "timber_room"
);

world.add_object(
    Object::new("kitchen_table", "kitchen table")
        .takeable(false)
        .openable()
    , "kitchen"
);

world.add_object(
    Object::new("attic_table", "table")
        .takeable(false)
        .openable()
    , "attic"
);

world.add_object(
    Object::new("sandwich_bag", "brown sack")
        .openable()
    , "kitchen_table"
);

world.add_object(
    Object::new("tool_chest", "group of tool chests")
        .takeable(false)
        .openable()
    , "maintenance_room"
);

world.add_object(
    Object::new("yellow_button", "yellow button")
        .takeable(false)
    , "maintenance_room"
);

world.add_object(
    Object::new("brown_button", "brown button")
        .takeable(false)
    , "maintenance_room"
);

world.add_object(
    Object::new("red_button", "red button")
        .takeable(false)
    , "maintenance_room"
);

world.add_object(
    Object::new("blue_button", "blue button")
        .takeable(false)
    , "maintenance_room"
);

world.add_object(
    Object::new("rug", "carpet")
        .takeable(false)
    , "living_room"
);

world.add_object(
    Object::new("chalice", "chalice")
        .openable()
    , "treasure_room"
);

world.add_object(
    Object::new("garlic", "clove of garlic")
    , "sandwich_bag"
);

world.add_object(
    Object::new("trident", "crystal trident")
    , "atlantis_room"
);

world.add_object(
    Object::new("cyclops", "cyclops")
        .takeable(false)
    , "cyclops_room"
);

world.add_object(
    Object::new("dam", "dam")
        .takeable(false)
    , "dam_room"
);

world.add_object(
    Object::new("trap_door", "trap door")
        .takeable(false)
        .openable()
    , "living_room"
);

world.add_object(
    Object::new("front_door", "door")
        .takeable(false)
        .openable()
    , "west_of_house"
);

world.add_object(
    Object::new("barrow_door", "stone door")
        .takeable(false)
        .openable()
    , "stone_barrow"
);

world.add_object(
    Object::new("barrow", "stone barrow")
        .takeable(false)
    , "stone_barrow"
);

world.add_object(
    Object::new("bottle", "glass bottle")
        .openable()
    , "kitchen_table"
);

world.add_object(
    Object::new("coffin", "gold coffin")
        .openable()
    , "egypt_room"
);

world.add_object(
    Object::new("pump", "hand-held air pump")
    , "reservoir_north"
);

world.add_object(
    Object::new("jade", "jade figurine")
    , "bat_room"
);

world.add_object(
    Object::new("knife", "nasty knife")
    , "attic_table"
);

world.add_object(
    Object::new("lamp", "brass lantern")
    , "living_room"
);

world.add_object(
    Object::new("emerald", "large emerald")
    , "buoy"
);

world.add_object(
    Object::new("advertisement", "leaflet")
    , "mailbox"
);

world.add_object(
    Object::new("leak", "leak")
        .takeable(false)
    , "maintenance_room"
);

world.add_object(
    Object::new("machine", "machine")
        .takeable(false)
        .openable()
    , "machine_room"
);

world.add_object(
    Object::new("mailbox", "small mailbox")
        .takeable(false)
        .openable()
    , "west_of_house"
);

world.add_object(
    Object::new("match", "matchbook")
    , "dam_lobby"
);

world.add_object(
    Object::new("painting", "painting")
    , "gallery"
);

world.add_object(
    Object::new("candles", "pair of candles")
    , "south_temple"
);

world.add_object(
    Object::new("leaves", "pile of leaves")
    , "grating_clearing"
);

world.add_object(
    Object::new("inflatable_boat", "pile of plastic")
    , "dam_base"
);

world.add_object(
    Object::new("bar", "platinum bar")
    , "loud_room"
);

world.add_object(
    Object::new("pot_of_gold", "pot of gold")
    , "end_of_rainbow"
);

world.add_object(
    Object::new("prayer", "prayer")
        .takeable(false)
    , "north_temple"
);

world.add_object(
    Object::new("railing", "wooden railing")
        .takeable(false)
    , "dome_room"
);

world.add_object(
    Object::new("rope", "rope")
    , "attic"
);

world.add_object(
    Object::new("sand", "sand")
        .takeable(false)
    , "sandy_cave"
);

world.add_object(
    Object::new("bracelet", "sapphire-encrusted bracelet")
    , "gas_room"
);

world.add_object(
    Object::new("screwdriver", "screwdriver")
    , "maintenance_room"
);

world.add_object(
    Object::new("shovel", "shovel")
    , "sandy_beach"
);

world.add_object(
    Object::new("scarab", "beautiful jeweled scarab")
    , "sandy_cave"
);

world.add_object(
    Object::new("large_bag", "large bag")
        .takeable(false)
    , "thief"
);

world.add_object(
    Object::new("stiletto", "stiletto")
    , "thief"
);

world.add_object(
    Object::new("machine_switch", "switch")
        .takeable(false)
    , "machine_room"
);

world.add_object(
    Object::new("wooden_door", "wooden door")
        .takeable(false)
        .openable()
    , "living_room"
);

world.add_object(
    Object::new("sword", "sword")
    , "living_room"
);

world.add_object(
    Object::new("map", "ancient map")
    , "trophy_case"
);

world.add_object(
    Object::new("boat_label", "tan label")
    , "inflated_boat"
);

world.add_object(
    Object::new("thief", "thief")
        .takeable(false)
        .openable()
    , "round_room"
);

world.add_object(
    Object::new("pedestal", "pedestal")
        .takeable(false)
        .openable()
    , "torch_room"
);

world.add_object(
    Object::new("torch", "torch")
    , "pedestal"
);

world.add_object(
    Object::new("guide", "tour guidebook")
    , "dam_lobby"
);

world.add_object(
    Object::new("troll", "troll")
        .takeable(false)
        .openable()
    , "troll_room"
);

world.add_object(
    Object::new("trunk", "trunk of jewels")
    , "reservoir"
);

world.add_object(
    Object::new("tube", "tube")
        .openable()
    , "maintenance_room"
);

world.add_object(
    Object::new("putty", "viscous material")
    , "tube"
);

world.add_object(
    Object::new("engravings", "wall with engravings")
        .takeable(false)
    , "engravings_cave"
);

world.add_object(
    Object::new("owners_manual", "ZORK owner's manual")
    , "studio"
);

world.add_object(
    Object::new("wrench", "wrench")
    , "maintenance_room"
);

world.add_object(
    Object::new("control_panel", "control panel")
        .takeable(false)
    , "dam_room"
);

world.add_object(
    Object::new("nest", "bird's nest")
        .openable()
    , "up_a_tree"
);

world.add_object(
    Object::new("egg", "jewel-encrusted egg")
        .openable()
    , "nest"
);

world.add_object(
    Object::new("canary", "golden clockwork canary")
    , "egg"
);

world.add_object(
    Object::new("broken_canary", "broken clockwork canary")
    , "broken_egg"
);

