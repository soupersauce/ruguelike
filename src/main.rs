use rand::Rng;
use tcod::colors::{self, Color};
use tcod::console::*;
use tcod::map::{FovAlgorithm, Map as FovMap};
use tcod::input::{self, Event, Mouse, Key};

mod lib;

pub use crate::lib::*;

pub use crate::lib::constants::*;

//pub use crate::lib::DeathCallback::*;

fn main() {
    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init();
    tcod::system::set_fps(LIMIT_FPS);

    let mut con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

    let mut player = Object::new(0, 0, '@', colors::WHITE, "Player", true);
    player.alive = true;
    player.fighter = Some(Fighter {
        max_hp:     30,
        hp:         30,
        defense:    2,
        power:      5,
        on_death:   DeathCallback::Player,
    });

    let mut objects = vec![player];

    let mut map = make_map(&mut objects);

    let mut fov_map = FovMap::new(MAP_WIDTH, MAP_HEIGHT);
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            fov_map.set(
                x,
                y,
                !map[x as usize][y as usize].block_sight,
                !map[x as usize][y as usize].blocked,
            );
        }
    }

    let mut previous_player_position = (-1, -1);

    let mut panel = Offscreen::new(SCREEN_WIDTH, PANEL_HEIGHT);

    let mut messages = vec![];

    let mut mouse = Default::default();
    let mut key = Default::default();

    message(
        &mut messages,
        "Welcome stranger! Prepare to perish.",
        colors::RED,
    );

    while !root.window_closed() {
        con.clear();

        let fov_recompute = previous_player_position != (objects[PLAYER].x, objects[PLAYER].y);

        match input::check_for_event(input::MOUSE | input::KEY_PRESS) {
            Some((_, Event::Mouse(m))) => mouse = m,
            Some((_, Event::Key(k))) => key = k,
            _ => key = Default::default(),
        }
        
        render_all(
            &mut root, 
            &mut con, 
            &mut panel,
            &objects, 
            &mut map,
            &messages,
            &mut fov_map,
            fov_recompute,
            mouse,
            );

        root.flush();

        let player = &mut objects[PLAYER];
        previous_player_position = (player.x, player.y);
        let player_action = handle_keys(key, &mut root, &map, &mut objects, &mut messages);

        if player_action == PlayerAction::Exit {
            break;
        }

        if objects[PLAYER].alive && player_action != PlayerAction::DidntTakeTurn {
            for id in 0..objects.len() {
                if objects[id].ai.is_some() {
                    ai_take_turn(id, &map, &mut objects, &fov_map, &mut messages);
                }
            }
        }
    }
}

fn handle_keys(
    key: Key,
    root: &mut Root,
    map: &Map,
    objects: &mut [Object],
    messages: &mut Messages
    ) -> PlayerAction {
    use PlayerAction::*;
    use tcod::input::KeyCode::*;

    let player_alive = objects[PLAYER].alive;
    match (key, player_alive) {
        (
        Key {
            code: Enter,
            alt: true,
            ..
        },
        _,
        ) => {
            // alt+enter to toggle fullscreen
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
            DidntTakeTurn
        }
        (Key { code: Escape, .. }, _) => Exit, //exit game

        (Key { code: Up, .. }, true) => {
            player_move_or_attack(0, -1, map, objects, messages);
            TookTurn
        }

        (Key { code: Down, .. }, true) => {
            player_move_or_attack(0, 1, map, objects, messages);
            TookTurn
        }

        (Key { code: Left, .. }, true) => {
            player_move_or_attack(-1, 0, map, objects, messages);
            TookTurn
        }

        (Key { code: Right, .. }, true) => {
            player_move_or_attack(1, 0, map, objects, messages);
            TookTurn
        }

        _ => DidntTakeTurn
    }
    
}

pub fn get_names_under_mouse(mouse: Mouse, objects: &[Object], fov_map: &FovMap) -> String {
    let (x, y) = (mouse.cx as i32, mouse.cy as i32);

    // create a list with the names of all objects at the mouse's coordinates and in FOV
    let names = objects
        .iter()
        .filter(|obj| {obj.pos() == (x, y) && fov_map.is_in_fov(obj.x, obj.y)})
        .map(|obj| obj.name.clone())
        .collect::<Vec<_>>();

    names.join(", ") // Join the names, separated by commas
}

pub fn make_map(objects: &mut Vec<Object>) -> Map {
    // fill map with "unblocked" tiles
    let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    let mut rooms = vec![];

    let mut starting_position = (0, 0);

    for _ in 0..MAX_ROOMS {
        // random width and height
        let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        // random position without going out of the boundaries of the map
        let x = rand::thread_rng().gen_range(0, MAP_WIDTH - w);
        let y = rand::thread_rng().gen_range(0, MAP_HEIGHT - h);

        let new_room = Rect::new(x, y, w, h);

        let failed = rooms
            .iter()
            .any(|other_room| new_room.intersects_with(other_room));

        if !failed {
            // valid room because no intersections

            //paint it to maps tiles
            create_room(new_room, &mut map);

            place_objects(new_room, objects);

            let (new_x, new_y) = new_room.center();

            if rooms.is_empty() {
                // this is the first room, where the player starts
                objects[PLAYER].set_pos(new_x, new_y);
            } else {
                //all rooms after the first
                //connect to previous with tunnels

                //center coordinates of previous room
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                // draw a coin (random bool value -- either true or false)
                if rand::random() {
                    // first horizontally, then vertically
                    create_h_tunnel(prev_x, new_x, prev_y, &mut map);
                    create_v_tunnel(prev_y, new_y, prev_x, &mut map);
                } else {
                    // first vertically, then horizontally
                    create_v_tunnel(prev_y, new_y, prev_x, &mut map);
                    create_h_tunnel(prev_x, new_x, prev_y, &mut map);
                    
                }
            }
            rooms.push(new_room);
        }
    }

    (map)
}

fn render_all(
            root:             &mut Root, 
            con:              &mut Offscreen, 
            panel:            &mut Offscreen,
            objects:          &[Object], 
            map:              &mut Map,
            messages:         &Messages,
            fov_map:          &mut FovMap,
            fov_recompute:    bool,
            mouse:          Mouse,
      ) {
    if fov_recompute {
        let player = &objects[PLAYER];
        fov_map.compute_fov(player.x, player.y, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO);
    }

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let visible = fov_map.is_in_fov(x, y);
            let wall = map[x as usize][y as usize].block_sight;
            let color = match (visible, wall) {
                //outisde fov
                (false, true) => COLOR_DARK_WALL,
                (false, false) => COLOR_DARK_GROUND,
                //inside fov
                (true, true) => COLOR_LIGHT_WALL,
                (true, false) => COLOR_LIGHT_GROUND,
            };
            let explored = &mut map[x as usize][y as usize].explored;
            if visible {
                // since it's visible, explore it
                *explored = true;
            }
            if *explored {
            con.set_char_background(x, y, color, BackgroundFlag::Set);
            }
        }
    }

    let mut to_draw: Vec<_> = objects
        .iter()
        .filter(|o| fov_map.is_in_fov(o.x, o.y))
        .collect();
    //sort so that non-blocking objects come first
    to_draw.sort_by(|o1, o2| { o1.blocks.cmp(&o2.blocks) });
    // draw the objects in the list
    for object in &to_draw {
        if fov_map.is_in_fov(object.x, object.y) {
            object.draw(con);
        }
    }
    blit(
        con, 
        (0, 0),
        (MAP_WIDTH, MAP_HEIGHT), 
        root, 
        (0, 0), 
        1.0,
        1.0,
    );
    // prepare to render GUI panel
    panel.set_default_background(colors::BLACK);
    panel.clear();

    // print the game messages, one line at a time
    let mut y = MSG_HEIGHT as i32;
    for &(ref msg, color) in messages.iter().rev() {
        let msg_height = panel.get_height_rect(MSG_X, y, MSG_WIDTH, 0, msg);
        y -= msg_height;
        if y < 0 {
            break;
        }
        panel.set_default_foreground(color);
        panel.print_rect(MSG_X, y, MSG_WIDTH, 0, msg);
    }

    // show the player's status
    let hp = objects[PLAYER].fighter.map_or(0, |f| f.hp);
    let max_hp = objects[PLAYER].fighter.map_or(0, |f| f.max_hp);
    render_bar(panel, 
                1, 
                1, 
                BAR_WIDTH, 
                "HP", 
                hp, 
                max_hp, 
                colors::LIGHT_RED, 
                colors::DARKER_RED,
    );

    panel.set_default_foreground(colors::LIGHT_GREY);
    panel.print_ex(
        1,
        0,
        BackgroundFlag::None,
        TextAlignment::Left,
        get_names_under_mouse(mouse, objects, fov_map),
    );

    blit(
        panel, 
        (0, 0), 
        (SCREEN_WIDTH, SCREEN_HEIGHT), 
        root, 
        (0, PANEL_Y), 
        1.0, 
        1.0
    );
    }

fn place_objects(room: Rect, objects: &mut Vec<Object>) {
    // choose random number of monsters
    let num_monsters = rand::thread_rng().gen_range(0, MAX_ROOM_MONSTERS + 1);

    for _ in 0..num_monsters {
        // chose random spot for this monster
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

        let mut monster = if rand::random::<f32>() < 0.8 { //80% chance of getting an orc
            // create an orc
            let mut orc = Object::new(x, y, 'o', colors::DESATURATED_GREEN, "Orc", true);
            orc.fighter = Some(Fighter {
                max_hp:     10,
                hp:         10,
                defense:    0,
                power:      3,
                on_death:   DeathCallback::Monster,
            });
            orc.ai = Some(Ai);
            orc
        } else {
            let mut troll = Object::new(x, y, 'T', colors::DARKER_GREEN, "Troll", true);
            troll.fighter = Some(Fighter {
                max_hp:     16,
                hp:         16,
                defense:    1,
                power:      4,
                on_death:    DeathCallback::Monster,
            });
            troll.ai = Some(Ai);
            troll
        };
        monster.alive = true;
        objects.push(monster);
    }
}

fn player_move_or_attack(dx: i32, dy: i32, map: &Map, objects: &mut [Object], messages: &mut Messages) {
    //coordinates player is moving to or attacking
    let x = objects[PLAYER].x + dx;
    let y = objects[PLAYER].y + dy;

    // try to find an attackable object
    let target_id = objects
        .iter()
        .position(|object| object.fighter.is_some() && object.pos() == (x, y));

    //attack if target found, move otherwise
    match target_id {
        Some(target_id) => {
            let (player, target) = mut_two(PLAYER, target_id, objects);
            player.attack(target, messages);
        }
        None => {
            move_by(PLAYER, dx, dy, map, objects);
        }
    }
}

pub fn ai_take_turn(
    monster_id: usize, 
    map: &Map, 
    objects: &mut [Object], 
    fov_map: &FovMap,
    messages: &mut Messages,
    ) {
    // a basic monster takes its turn. If you can see it, it can see you
    let (monster_x, monster_y) = objects[monster_id].pos();
    if fov_map.is_in_fov(monster_x, monster_y) {
        if objects[monster_id].distance_to(&objects[PLAYER]) >= 2.0 {
            // move towards player if far away
            let (player_x, player_y) = objects[PLAYER].pos();
            move_towards(monster_id, player_x, player_y, map, objects);
        } else if objects[PLAYER].fighter.map_or(false, |f| f.hp > 0) {
            // close enough to attack! (if the player is still alive)
            let (monster, player) = mut_two(monster_id, PLAYER, objects);
            monster.attack(player, messages);
        }
    }
}

