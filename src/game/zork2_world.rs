// ZORK2 - 86 rooms, 145 objects

world.add_room(Room::new("inside_barrow", "Inside the Barrow")
    .with_exit("south", "narrow_tunnel")
);

world.add_room(Room::new("narrow_tunnel", "Narrow Tunnel")
    .with_exit("north", "inside_barrow")
    .with_exit("south", "foot_bridge")
    .with_exit("cross", "foot_bridge")
);

world.add_room(Room::new("foot_bridge", "Foot Bridge")
    .with_exit("north", "narrow_tunnel")
    .with_exit("south", "great_cavern")
);

world.add_room(Room::new("great_cavern", "Great Cavern")
    .with_exit("ne", "foot_bridge")
    .with_exit("sw", "shallow_ford")
);

world.add_room(Room::new("shallow_ford", "Shallow Ford")
    .with_exit("north", "great_cavern")
    .with_exit("south", "dark_tunnel")
    .with_exit("cross", "dark_tunnel")
);

world.add_room(Room::new("dark_tunnel", "Dark Tunnel")
    .with_exit("ne", "shallow_ford")
    .with_exit("se", "garden_north")
    .with_exit("sw", "stream_path")
);

world.add_room(Room::new("garden_north", "North End of Garden")
    .with_exit("in", "gazebo_room")
    .with_exit("north", "dark_tunnel")
    .with_exit("south", "formal_garden")
);

world.add_room(Room::new("gazebo_room", "Gazebo")
    .with_exit("out", "garden_north")
);

world.add_room(Room::new("formal_garden", "Formal Garden")
    .with_exit("west", "stream_path")
    .with_exit("north", "garden_north")
    .with_exit("south", "topiary_room")
);

world.add_room(Room::new("topiary_room", "Topiary")
    .with_exit("west", "carousel_room")
    .with_exit("north", "formal_garden")
);

world.add_room(Room::new("stream_path", "Path Near Stream")
    .with_exit("east", "formal_garden")
    .with_exit("ne", "dark_tunnel")
    .with_exit("sw", "carousel_room")
);

world.add_room(Room::new("marble_hall", "Marble Hall")
    .with_exit("north", "deep_ford")
    .with_exit("south", "carousel_room")
);

world.add_room(Room::new("deep_ford", "Deep Ford")
    .with_exit("north", "ravine_ledge")
    .with_exit("up", "ravine_ledge")
    .with_exit("south", "marble_hall")
);

world.add_room(Room::new("ravine_ledge", "Ledge in Ravine")
    .with_exit("up", "tiny_room")
    .with_exit("south", "deep_ford")
    .with_exit("west", "ledge_tunnel")
    .with_exit("down", "deep_ford")
);

world.add_room(Room::new("ledge_tunnel", "End of Ledge")
    .with_exit("east", "ravine_ledge")
    .with_exit("in", "dragon_room")
    .with_exit("north", "dragon_room")
);

world.add_room(Room::new("dragon_room", "Dragon Room")
    .with_exit("east", "ledge_tunnel")
    .with_exit("west", "fresco_room")
    .with_exit("south", "stone_bridge")
    .with_exit("cross", "stone_bridge")
);

world.add_room(Room::new("dragon_lair", "Dragon's Lair")
    .with_exit("south", "dragon_room")
    .with_exit("out", "dragon_room")
);

world.add_room(Room::new("fresco_room", "Fresco Room")
    .with_exit("east", "dragon_room")
    .with_exit("west", "bank_entrance")
);

world.add_room(Room::new("stone_bridge", "Stone Bridge")
    .with_exit("north", "dragon_room")
    .with_exit("south", "cool_room")
);

world.add_room(Room::new("cool_room", "Cool Room")
    .with_exit("se", "carousel_room")
    .with_exit("north", "stone_bridge")
    .with_exit("west", "glacier_room")
    .with_exit("cross", "stone_bridge")
);

world.add_room(Room::new("glacier_room", "Ice Room")
    .with_exit("east", "cool_room")
    .with_exit("up", "lava_tube")
);

world.add_room(Room::new("lava_tube", "Lava Tube")
    .with_exit("down", "glacier_room")
    .with_exit("up", "volcano_view")
    .with_exit("south", "cobwebby_corridor")
);

world.add_room(Room::new("cobwebby_corridor", "Cobwebby Corridor")
    .with_exit("in", "lava_tube")
    .with_exit("up", "lava_tube")
    .with_exit("north", "lava_tube")
    .with_exit("ne", "carousel_room")
    .with_exit("sw", "guardian_room")
    .with_exit("down", "guardian_room")
);

world.add_room(Room::new("room_8", "Room 8")
    .with_exit("east", "carousel_room")
);

world.add_room(Room::new("menhir_room", "Menhir Room")
    .with_exit("north", "carousel_room")
    .with_exit("south", "stairway_top")
);

world.add_room(Room::new("kennel", "Kennel")
);

world.add_room(Room::new("stairway_top", "Stairway")
    .with_exit("down", "diamond_5")
    .with_exit("north", "menhir_room")
);

world.add_room(Room::new("diamond_1", "Oddly-angled Room")
    .with_exit("se", "diamond_5")
);

world.add_room(Room::new("diamond_2", "Oddly-angled Room")
    .with_exit("south", "diamond_5")
    .with_exit("se", "diamond_6")
    .with_exit("sw", "diamond_4")
);

world.add_room(Room::new("diamond_3", "Oddly-angled Room")
    .with_exit("sw", "diamond_5")
);

world.add_room(Room::new("diamond_4", "Oddly-angled Room")
    .with_exit("ne", "diamond_2")
    .with_exit("se", "diamond_8")
    .with_exit("east", "diamond_5")
);

world.add_room(Room::new("diamond_5", "Oddly-angled Room")
    .with_exit("nw", "diamond_1")
    .with_exit("north", "diamond_2")
    .with_exit("ne", "diamond_3")
    .with_exit("west", "diamond_4")
    .with_exit("east", "diamond_6")
    .with_exit("sw", "diamond_7")
    .with_exit("south", "diamond_8")
    .with_exit("se", "diamond_9")
    .with_exit("down", "cerberus_room")
    .with_exit("up", "stairway_top")
);

world.add_room(Room::new("diamond_6", "Oddly-angled Room")
    .with_exit("west", "diamond_5")
    .with_exit("nw", "diamond_2")
    .with_exit("sw", "diamond_8")
);

world.add_room(Room::new("diamond_7", "Oddly-angled Room")
    .with_exit("ne", "diamond_5")
);

world.add_room(Room::new("diamond_8", "Oddly-angled Room")
    .with_exit("north", "diamond_5")
    .with_exit("nw", "diamond_4")
    .with_exit("ne", "diamond_6")
);

world.add_room(Room::new("diamond_9", "Oddly-angled Room")
    .with_exit("nw", "diamond_5")
);

world.add_room(Room::new("cerberus_room", "Cerberus Room")
    .with_exit("up", "diamond_5")
);

world.add_room(Room::new("crypt_anteroom", "Crypt Anteroom")
    .with_exit("west", "cerberus_room")
);

world.add_room(Room::new("crypt_room", "Crypt")
);

world.add_room(Room::new("zork3", "Landing")
);

world.add_room(Room::new("guardian_room", "Guarded Room")
    .with_exit("north", "cobwebby_corridor")
);

world.add_room(Room::new("wizards_workshop", "Wizard's Workshop")
    .with_exit("west", "workbench_room")
    .with_exit("south", "trophy_room")
);

world.add_room(Room::new("workbench_room", "Wizard's Workroom")
    .with_exit("east", "wizards_workshop")
    .with_exit("south", "pentagram_room")
    .with_exit("west", "aquarium_room")
);

world.add_room(Room::new("trophy_room", "Trophy Room")
    .with_exit("north", "wizards_workshop")
);

world.add_room(Room::new("pentagram_room", "Pentagram Room")
    .with_exit("north", "workbench_room")
);

world.add_room(Room::new("aquarium_room", "Aquarium Room")
    .with_exit("east", "workbench_room")
    .with_exit("in", "in_aquarium")
    .with_exit("south", "wizards_quarters")
);

world.add_room(Room::new("in_aquarium", "Murky Room")
    .with_exit("out", "aquarium_room")
);

world.add_room(Room::new("wizards_quarters", "Wizard's Quarters")
    .with_exit("north", "aquarium_room")
);

world.add_room(Room::new("carousel_room", "Carousel Room")
    .with_exit("north", "marble_hall")
    .with_exit("ne", "stream_path")
    .with_exit("east", "topiary_room")
    .with_exit("se", "riddle_room")
    .with_exit("south", "menhir_room")
    .with_exit("sw", "cobwebby_corridor")
    .with_exit("west", "room_8")
    .with_exit("nw", "cool_room")
);

world.add_room(Room::new("riddle_room", "Riddle Room")
    .with_exit("down", "carousel_room")
    .with_exit("nw", "carousel_room")
);

world.add_room(Room::new("pearl_room", "Pearl Room")
    .with_exit("east", "well_bottom")
    .with_exit("west", "riddle_room")
);

world.add_room(Room::new("volcano_bottom", "Volcano Bottom")
    .with_exit("north", "lava_room")
);

world.add_room(Room::new("vair_1", "Volcano Core")
);

world.add_room(Room::new("vair_2", "Volcano Near Small Ledge")
    .with_exit("west", "ledge_1")
    .with_exit("land", "ledge_1")
);

world.add_room(Room::new("vair_3", "Volcano by Viewing Ledge")
);

world.add_room(Room::new("vair_4", "Volcano Near Wide Ledge")
    .with_exit("land", "ledge_2")
    .with_exit("west", "ledge_2")
);

world.add_room(Room::new("ledge_1", "Narrow Ledge")
    .with_exit("south", "library")
);

world.add_room(Room::new("library", "Library")
    .with_exit("north", "ledge_1")
    .with_exit("out", "ledge_1")
);

world.add_room(Room::new("volcano_view", "Volcano View")
    .with_exit("east", "lava_tube")
);

world.add_room(Room::new("ledge_2", "Wide Ledge")
    .with_exit("south", "safe_room")
);

world.add_room(Room::new("safe_room", "Dusty Room")
    .with_exit("north", "ledge_2")
);

world.add_room(Room::new("lava_room", "Lava Room")
    .with_exit("south", "volcano_bottom")
    .with_exit("east", "glacier_room")
);

world.add_room(Room::new("magnet_room", "Low Room")
);

world.add_room(Room::new("machine_room", "Machine Room")
    .with_exit("west", "magnet_room")
    .with_exit("south", "cage_room")
);

world.add_room(Room::new("cage_room", "Dingy Closet")
    .with_exit("out", "machine_room")
    .with_exit("north", "machine_room")
);

world.add_room(Room::new("in_cage", "Cage")
);

world.add_room(Room::new("well_top", "Top of Well")
    .with_exit("east", "tea_room")
);

world.add_room(Room::new("well_bottom", "Circular Room")
    .with_exit("west", "pearl_room")
);

world.add_room(Room::new("tea_room", "Tea Room")
    .with_exit("west", "well_top")
    .with_exit("nw", "magnet_room")
);

world.add_room(Room::new("posts_room", "Posts Room")
    .with_exit("east", "pool_room")
);

world.add_room(Room::new("pool_room", "Pool Room")
    .with_exit("out", "posts_room")
    .with_exit("west", "posts_room")
);

world.add_room(Room::new("bank_entrance", "Bank Entrance")
    .with_exit("nw", "teller_west")
    .with_exit("ne", "teller_east")
    .with_exit("east", "fresco_room")
);

world.add_room(Room::new("teller_west", "West Teller's Room")
    .with_exit("north", "viewing_west")
    .with_exit("south", "bank_entrance")
    .with_exit("west", "depository")
);

world.add_room(Room::new("teller_east", "East Teller's Room")
    .with_exit("north", "viewing_east")
    .with_exit("south", "bank_entrance")
    .with_exit("east", "depository")
);

world.add_room(Room::new("viewing_west", "West Viewing Room")
    .with_exit("south", "bank_entrance")
);

world.add_room(Room::new("viewing_east", "East Viewing Room")
    .with_exit("south", "bank_entrance")
);

world.add_room(Room::new("small_room", "Small Room")
);

world.add_room(Room::new("vault", "Vault")
);

world.add_room(Room::new("depository", "Safety Depository")
    .with_exit("south", "office")
);

world.add_room(Room::new("office", "Chairman's Office")
    .with_exit("north", "depository")
);

world.add_room(Room::new("dreary_room", "Dreary Room")
);

world.add_room(Room::new("tiny_room", "Tiny Room")
    .with_exit("down", "ravine_ledge")
);

world.add_room(Room::new("dead_palantir_1", "Room of Red Mist")
    .with_exit("west", "dead_palantir_2")
);

world.add_room(Room::new("dead_palantir_2", "Room of Blue Mist")
    .with_exit("west", "dead_palantir_3")
);

world.add_room(Room::new("dead_palantir_3", "Room of White Mist")
    .with_exit("west", "dead_palantir_4")
);

world.add_room(Room::new("dead_palantir_4", "Room of Black Mist")
);

world.add_object(
    Object::new("hedges", "hedge")
        .takeable(false)
    , "topiary_room"
);

world.add_object(
    Object::new("chest", "rotten wooden chest")
        .openable()
    , "dragon_lair"
);

world.add_object(
    Object::new("statuette", "golden dragon statuette")
    , "chest"
);

world.add_object(
    Object::new("dragon", "huge red dragon")
        .takeable(false)
    , "dragon_room"
);

world.add_object(
    Object::new("princess", "beautiful princess")
        .takeable(false)
    , "dragon_lair"
);

world.add_object(
    Object::new("collar", "gigantic dog collar")
    , "kennel"
);

world.add_object(
    Object::new("cerberus", "three-headed dog")
        .takeable(false)
        .openable()
    , "cerberus_room"
);

world.add_object(
    Object::new("heads", "set of poled heads")
        .takeable(false)
    , "crypt_room"
);

world.add_object(
    Object::new("crypt", "marble crypt")
        .takeable(false)
    , "crypt_room"
);

world.add_object(
    Object::new("door_keeper", "lizard")
        .takeable(false)
    , "guardian_room"
);

world.add_object(
    Object::new("arcana", "arcane item")
        .takeable(false)
    , "workbench_room"
);

world.add_object(
    Object::new("workbench", "Wizard's workbench")
        .takeable(false)
        .openable()
    , "workbench_room"
);

world.add_object(
    Object::new("stand_1", "ruby stand")
        .takeable(false)
        .openable()
    , "workbench"
);

world.add_object(
    Object::new("stand_2", "sapphire stand")
        .takeable(false)
        .openable()
    , "workbench"
);

world.add_object(
    Object::new("stand_3", "diamond stand")
        .takeable(false)
        .openable()
    , "workbench"
);

world.add_object(
    Object::new("degree", "degree")
        .takeable(false)
    , "trophy_room"
);

world.add_object(
    Object::new("wands", "set of used wands")
        .takeable(false)
    , "trophy_room"
);

world.add_object(
    Object::new("trophy_sword", "nicked swords")
        .takeable(false)
    , "trophy_room"
);

world.add_object(
    Object::new("trophy_bottles", "small bottles")
        .takeable(false)
    , "trophy_room"
);

world.add_object(
    Object::new("warning_label", "warning label")
        .takeable(false)
    , "trophy_room"
);

world.add_object(
    Object::new("wizard_case", "Wizard's trophy cabinet")
        .takeable(false)
        .openable()
    , "trophy_room"
);

world.add_object(
    Object::new("pentagram", "pentagram")
        .takeable(false)
        .openable()
    , "pentagram_room"
);

world.add_object(
    Object::new("aquarium", "aquarium")
        .takeable(false)
        .openable()
    , "aquarium_room"
);

world.add_object(
    Object::new("palantir_3", "clear crystal sphere")
    , "in_aquarium"
);

world.add_object(
    Object::new("serpent", "baby sea serpent")
        .takeable(false)
    , "aquarium"
);

world.add_object(
    Object::new("riddle_door", "stone door")
        .takeable(false)
        .openable()
    , "riddle_room"
);

world.add_object(
    Object::new("match", "matchbook")
    , "gazebo_table"
);

world.add_object(
    Object::new("balloon", "basket")
        .takeable(false)
        .openable()
    , "volcano_bottom"
);

world.add_object(
    Object::new("safe", "box")
        .takeable(false)
        .openable()
    , "safe_room"
);

world.add_object(
    Object::new("braided_wire", "braided wire")
        .takeable(false)
    , "balloon"
);

world.add_object(
    Object::new("brick", "brick")
        .openable()
    , "marble_hall"
);

world.add_object(
    Object::new("round_button", "round button")
        .takeable(false)
    , "machine_room"
);

world.add_object(
    Object::new("square_button", "square button")
        .takeable(false)
    , "machine_room"
);

world.add_object(
    Object::new("triangular_button", "triangular button")
        .takeable(false)
    , "machine_room"
);

world.add_object(
    Object::new("card", "card")
    , "safe"
);

world.add_object(
    Object::new("cloth_bag", "cloth bag")
        .takeable(false)
    , "balloon"
);

world.add_object(
    Object::new("crown", "gaudy crown")
    , "safe"
);

world.add_object(
    Object::new("violin", "fancy violin")
    , "iron_box"
);

world.add_object(
    Object::new("ice", "glacier")
        .takeable(false)
    , "glacier_room"
);

world.add_object(
    Object::new("flask", "stoppered glass flask filled with liquid")
    , "pool_room"
);

world.add_object(
    Object::new("robot_label", "green piece of paper")
    , "magnet_room"
);

world.add_object(
    Object::new("slot", "hole")
        .takeable(false)
        .openable()
    , "safe_room"
);

world.add_object(
    Object::new("lamp", "lamp")
    , "inside_barrow"
);

world.add_object(
    Object::new("alice_table", "large oblong table")
        .takeable(false)
        .openable()
    , "tea_room"
);

world.add_object(
    Object::new("pearl", "pearl necklace")
    , "pearl_room"
);

world.add_object(
    Object::new("eat_me_cake", "cake frosted with green letters")
    , "alice_table"
);

world.add_object(
    Object::new("blue_icing", "cake frosted with blue letters")
    , "alice_table"
);

world.add_object(
    Object::new("orange_icing", "cake frosted with orange letters")
    , "alice_table"
);

world.add_object(
    Object::new("red_icing", "cake frosted with red letters")
    , "alice_table"
);

world.add_object(
    Object::new("leak", "leak")
        .takeable(false)
    , "pool_room"
);

world.add_object(
    Object::new("pool", "pool of tears")
        .takeable(false)
    , "pool_room"
);

world.add_object(
    Object::new("stamp", "Flathead stamp")
    , "purple_book"
);

world.add_object(
    Object::new("green_book", "green book")
        .openable()
    , "library"
);

world.add_object(
    Object::new("blue_book", "blue book")
        .openable()
    , "library"
);

world.add_object(
    Object::new("white_book", "white book")
        .openable()
    , "library"
);

world.add_object(
    Object::new("purple_book", "purple book")
        .openable()
    , "library"
);

world.add_object(
    Object::new("receptacle", "receptacle")
        .takeable(false)
        .openable()
    , "balloon"
);

world.add_object(
    Object::new("robot", "robot")
        .takeable(false)
        .openable()
    , "magnet_room"
);

world.add_object(
    Object::new("ruby", "ruby")
    , "lava_room"
);

world.add_object(
    Object::new("iron_box", "steel box")
        .openable()
    , "carousel_room"
);

world.add_object(
    Object::new("cage", "solid steel cage")
        .takeable(false)
    , "cage_room"
);

world.add_object(
    Object::new("candy", "package of candy")
    , "pool_room"
);

world.add_object(
    Object::new("top_etchings", "wall with etchings")
        .takeable(false)
    , "well_top"
);

world.add_object(
    Object::new("bottom_etchings", "wall with etchings")
        .takeable(false)
    , "well_bottom"
);

world.add_object(
    Object::new("fuse", "black string")
    , "cobwebby_corridor"
);

world.add_object(
    Object::new("bucket", "wooden bucket")
        .takeable(false)
        .openable()
    , "well_bottom"
);

world.add_object(
    Object::new("posts", "group of wooden posts")
        .takeable(false)
    , "posts_room"
);

world.add_object(
    Object::new("bills", "stack of zorkmid bills")
    , "vault"
);

world.add_object(
    Object::new("portrait", "portrait of J. Pierpont Flathead")
    , "office"
);

world.add_object(
    Object::new("bank_brochure", "bank brochure")
    , "depository"
);

world.add_object(
    Object::new("cube", "large stone cube")
        .takeable(false)
    , "depository"
);

world.add_object(
    Object::new("curtain", "shimmering curtain of light")
        .takeable(false)
    , "depository"
);

world.add_object(
    Object::new("deposit_box", "safety deposit box")
    , "gnome_of_zurich"
);

world.add_object(
    Object::new("lid_1", "metal lid")
        .takeable(false)
        .openable()
    , "tiny_room"
);

world.add_object(
    Object::new("lid_2", "metal lid")
        .takeable(false)
        .openable()
    , "dreary_room"
);

world.add_object(
    Object::new("ptable", "table")
        .takeable(false)
        .openable()
    , "dreary_room"
);

world.add_object(
    Object::new("pcrack", "narrow crack")
        .takeable(false)
    , "dreary_room"
);

world.add_object(
    Object::new("keyhole_1", "keyhole")
        .takeable(false)
        .openable()
    , "tiny_room"
);

world.add_object(
    Object::new("keyhole_2", "keyhole")
        .takeable(false)
        .openable()
    , "dreary_room"
);

world.add_object(
    Object::new("letter_opener", "letter opener")
    , "gazebo_table"
);

world.add_object(
    Object::new("palantir_2", "blue crystal sphere")
    , "dreary_room"
);

world.add_object(
    Object::new("gazebo_table", "table")
        .takeable(false)
        .openable()
    , "gazebo_room"
);

world.add_object(
    Object::new("newspaper", "newspaper")
    , "gazebo_table"
);

world.add_object(
    Object::new("place_mat", "place mat")
        .openable()
    , "gazebo_table"
);

world.add_object(
    Object::new("teapot", "china teapot")
        .openable()
    , "gazebo_table"
);

world.add_object(
    Object::new("gold_key", "delicate gold key")
    , "unicorn"
);

world.add_object(
    Object::new("ribbon", "ribbon")
        .takeable(false)
    , "unicorn"
);

world.add_object(
    Object::new("wand", "Wizard's magic wand")
    , "wizard"
);

world.add_object(
    Object::new("sword", "elvish sword")
    , "inside_barrow"
);

