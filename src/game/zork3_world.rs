// ZORK3 - 35 rooms, 44 objects

world.add_room(Room::new("cp_ante", "Royal Puzzle Entrance")
    .with_exit("west", "cp_out")
    .with_exit("north", "museum_entrance")
    .with_exit("up", "museum_entrance")
);

world.add_room(Room::new("cp_out", "Side Room")
    .with_exit("north", "cp_ante")
    .with_exit("up", "cp_ante")
);

world.add_room(Room::new("cp", "Room in a Puzzle")
);

world.add_room(Room::new("mrd", "Hallway")
    .with_exit("north", "front_door")
    .with_exit("ne", "front_door")
    .with_exit("nw", "front_door")
);

world.add_room(Room::new("mrg", "Hallway")
);

world.add_room(Room::new("mrc", "Hallway")
);

world.add_room(Room::new("mrb", "Hallway")
);

world.add_room(Room::new("mra", "Hallway")
    .with_exit("south", "mreye")
);

world.add_room(Room::new("mrde", "Narrow Room")
    .with_exit("north", "front_door")
    .with_exit("south", "mrg")
);

world.add_room(Room::new("mrdw", "Narrow Room")
    .with_exit("north", "front_door")
    .with_exit("south", "mrg")
);

world.add_room(Room::new("mrge", "Narrow Room")
    .with_exit("north", "mrd")
    .with_exit("south", "mrc")
);

world.add_room(Room::new("mrgw", "Narrow Room")
    .with_exit("north", "mrd")
    .with_exit("south", "mrc")
);

world.add_room(Room::new("mrce", "Narrow Room")
    .with_exit("north", "mrg")
    .with_exit("south", "mrb")
);

world.add_room(Room::new("mrcw", "Narrow Room")
    .with_exit("north", "mrg")
    .with_exit("south", "mrb")
);

world.add_room(Room::new("mrbe", "Narrow Room")
    .with_exit("north", "mrc")
    .with_exit("south", "mra")
);

world.add_room(Room::new("mrbw", "Narrow Room")
    .with_exit("north", "mrc")
    .with_exit("south", "mra")
);

world.add_room(Room::new("mrae", "Narrow Room")
    .with_exit("north", "mrb")
    .with_exit("south", "mreye")
);

world.add_room(Room::new("mraw", "Narrow Room")
    .with_exit("north", "mrb")
    .with_exit("south", "mreye")
);

world.add_room(Room::new("in_mirror", "Inside Mirror")
);

world.add_room(Room::new("mr_ante", "Button Room")
    .with_exit("north", "mreye")
);

world.add_room(Room::new("mreye", "Beam Room")
    .with_exit("south", "mr_ante")
);

world.add_room(Room::new("mstairs", "Engravings Room")
    .with_exit("sw", "damp_passage")
    .with_exit("se", "dead_end")
);

world.add_room(Room::new("dead_end", "Dead End")
    .with_exit("west", "damp_passage")
    .with_exit("nw", "mstairs")
);

world.add_room(Room::new("damp_passage", "Damp Passage")
    .with_exit("west", "junction")
    .with_exit("east", "dead_end")
    .with_exit("ne", "mstairs")
);

world.add_room(Room::new("east_corridor", "East Corridor")
    .with_exit("north", "north_corridor")
    .with_exit("south", "south_corridor")
);

world.add_room(Room::new("west_corridor", "West Corridor")
    .with_exit("north", "north_corridor")
    .with_exit("south", "south_corridor")
);

world.add_room(Room::new("south_corridor", "South Corridor")
    .with_exit("east", "east_corridor")
    .with_exit("west", "west_corridor")
    .with_exit("south", "behind_door")
);

world.add_room(Room::new("behind_door", "Narrow Corridor")
    .with_exit("north", "south_corridor")
);

world.add_room(Room::new("front_door", "Dungeon Entrance")
    .with_exit("se", "mrde")
    .with_exit("sw", "mrdw")
);

world.add_room(Room::new("north_corridor", "North Corridor")
    .with_exit("east", "east_corridor")
    .with_exit("west", "west_corridor")
    .with_exit("north", "parapet")
);

world.add_room(Room::new("parapet", "Parapet")
    .with_exit("south", "north_corridor")
);

world.add_room(Room::new("cell", "Prison Cell")
);

world.add_room(Room::new("prison_cell", "Prison Cell")
);

world.add_room(Room::new("good_cell", "Prison Cell")
);

world.add_room(Room::new("nirvana", "Treasury of Zork")
);

world.add_object(
    Object::new("warning_note", "warning note")
    , "cp_ante"
);

world.add_object(
    Object::new("dungeon_master", "dungeon master")
        .takeable(false)
    , "behind_door"
);

world.add_object(
    Object::new("runes", "runes")
        .takeable(false)
    , "mstairs"
);

world.add_object(
    Object::new("t_bar", "T-bar")
        .takeable(false)
    , "in_mirror"
);

world.add_object(
    Object::new("black_panel", "black panel")
        .takeable(false)
    , "in_mirror"
);

world.add_object(
    Object::new("compass_arrow", "compass arrow")
        .takeable(false)
    , "in_mirror"
);

world.add_object(
    Object::new("dial_button", "large button")
        .takeable(false)
    , "parapet"
);

world.add_object(
    Object::new("long_pole", "long pole")
        .takeable(false)
    , "in_mirror"
);

world.add_object(
    Object::new("mahogany_panel", "mahogany panel")
        .takeable(false)
    , "in_mirror"
);

world.add_object(
    Object::new("pine_panel", "pine panel")
        .takeable(false)
    , "in_mirror"
);

world.add_object(
    Object::new("beam", "red beam of light")
        .takeable(false)
        .openable()
    , "mreye"
);

world.add_object(
    Object::new("red_button", "red button")
        .takeable(false)
    , "mr_ante"
);

world.add_object(
    Object::new("red_panel", "red panel")
        .takeable(false)
    , "in_mirror"
);

world.add_object(
    Object::new("short_pole", "short pole")
        .takeable(false)
    , "in_mirror"
);

world.add_object(
    Object::new("sundial", "sundial")
        .takeable(false)
    , "parapet"
);

world.add_object(
    Object::new("yellow_panel", "yellow panel")
        .takeable(false)
    , "in_mirror"
);

world.add_object(
    Object::new("white_panel", "white panel")
        .takeable(false)
    , "in_mirror"
);

world.add_object(
    Object::new("wooden_bar", "wooden bar")
        .takeable(false)
    , "in_mirror"
);

world.add_object(
    Object::new("cp_hole", "hole")
        .takeable(false)
    , "cp_ante"
);

