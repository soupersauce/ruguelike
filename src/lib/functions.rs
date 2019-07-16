use tcod::input::{Mouse, Key};
use tcod::map::Map as FovMap;
use tcod::colors::*;
use tcod::input::{self, Event};
use rand::Rng;

use super::*;

pub use super::constants::*;

pub fn handle_keys(
    key: Key,
    tcod: &mut Tcod,
    map: &mut Map,
    objects: &mut Vec<Object>,
    messages: &mut Messages,
    inventory: &mut Vec<Object>,
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
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
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

        (Key { printable: 'g', .. }, true) => {
            //pick up item
            let item_id = objects
                .iter()
                .position(|object| object.pos() == objects[PLAYER].pos() && object.item.is_some());
            if let Some(item_id) = item_id {
                pick_item_up(item_id, objects, inventory, messages);
            }
            DidntTakeTurn
        }

        (Key { printable: 'i', .. }, true) => {
            // show the inventory
            let inventory_index = inventory_menu(
                inventory,
                "Press the key next to an item to use it, or any other to cancel. \n",
                &mut tcod.root,
            );

            if let Some(inventory_index) = inventory_index {
                use_item(inventory_index, inventory, objects, messages, map, tcod);
            }
            DidntTakeTurn
        }
        (Key { printable: 'd', .. }, true) => {
            // show the inventory
            let inventory_index = inventory_menu(
                inventory,
                "Press the key next to an item to drop it, or any other to cancel. \n",
                &mut tcod.root,
            );

            if let Some(inventory_index) = inventory_index {
                drop_item(inventory_index, inventory, objects, messages, );
            }
            DidntTakeTurn
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

            place_objects(new_room, objects, &map);

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

pub fn render_all(
    tcod:               &mut Tcod,
    objects:            &[Object], 
    map:                &mut Map,
    messages:           &Messages,
    fov_recompute:      bool,
  ) {
    if fov_recompute {
        let player = &objects[PLAYER];
        tcod.fov
            .compute_fov(player.x, player.y, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO);
    }

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let visible = tcod.fov.is_in_fov(x, y);
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
            tcod.con.set_char_background(x, y, color, BackgroundFlag::Set);
            }
        }
    }

    let mut to_draw: Vec<_> = objects
        .iter()
        .filter(|o| tcod.fov.is_in_fov(o.x, o.y))
        .collect();
    //sort so that non-blocking objects come first
    to_draw.sort_by(|o1, o2| { o1.blocks.cmp(&o2.blocks) });
    // draw the objects in the list
    for object in &to_draw {
        if tcod.fov.is_in_fov(object.x, object.y) {
            object.draw(&mut tcod.con);
        }
    }
    blit(
        &mut tcod.con, 
        (0, 0),
        (MAP_WIDTH, MAP_HEIGHT), 
        &mut tcod.root, 
        (0, 0), 
        1.0,
        1.0,
    );
    // prepare to render GUI panel
    tcod.panel.set_default_background(colors::BLACK);
    tcod.panel.clear();

    // print the game messages, one line at a time
    let mut y = MSG_HEIGHT as i32;
    for &(ref msg, color) in messages.iter().rev() {
        let msg_height = tcod.panel.get_height_rect(MSG_X, y, MSG_WIDTH, 0, msg);
        y -= msg_height;
        if y < 0 {
            break;
        }
        tcod.panel.set_default_foreground(color);
        tcod.panel.print_rect(MSG_X, y, MSG_WIDTH, 0, msg);
    }

    // show the player's status
    let hp = objects[PLAYER].fighter.map_or(0, |f| f.hp);
    let max_hp = objects[PLAYER].fighter.map_or(0, |f| f.max_hp);
    render_bar(&mut tcod.panel, 
                1, 
                1, 
                BAR_WIDTH, 
                "HP", 
                hp, 
                max_hp, 
                colors::LIGHT_RED, 
                colors::DARKER_RED,
    );

    tcod.panel.set_default_foreground(colors::LIGHT_GREY);
    tcod.panel.print_ex(
        1,
        0,
        BackgroundFlag::None,
        TextAlignment::Left,
        get_names_under_mouse(tcod.mouse, objects, &tcod.fov),
    );

    blit(
        &tcod.panel, 
        (0, 0), 
        (SCREEN_WIDTH, SCREEN_HEIGHT), 
        &mut tcod.root, 
        (0, PANEL_Y), 
        1.0, 
        1.0
    );
}

fn place_objects(room: Rect, objects: &mut Vec<Object>, map: &Map) {
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
            orc.ai = Some(Ai::Basic);
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
            troll.ai = Some(Ai::Basic);
            troll
        };
        monster.alive = true;
        objects.push(monster);
    }

    let num_items = rand::thread_rng().gen_range(0, constants::MAX_ROOM_ITEMS + 1);

    for _ in 0..num_items {
        // choose random spot for this item
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

        // only place it if the tile is not blocked
        if !is_blocked(x, y, &map, objects) {
            let dice = rand::random::<f32>();
            let item = if dice < 0.7 {
                // create a healing object
                let mut object = Object::new(x, y, '!', colors::VIOLET, "healing potion", false,);
                object.item = Some(Item::Heal);
                object
            } else if dice < 0.7 + 0.10 {
                // create a lightning bolt scroll (30% chance)
                let mut object = Object::new(
                    x,
                    y,
                    '#',
                    colors::LIGHT_YELLOW,
                    "Scroll of lightning bolt",
                    false,
                );
                object.item = Some(Item::Lightning);
                object
            } else if dice < 0.7 + 0.1 + 0.1 {
                let mut object = Object::new(
                    x,
                    y,
                    '#',
                    colors::LIGHT_YELLOW,
                    "Scroll of confusion",
                    false,
                );
                object.item = Some(Item::Confuse);
                object
            } else {
                // create fireball scroll (10% chance)
                let mut object =
                    Object::new(x, y, '#', colors::LIGHT_YELLOW, "Scroll of fireball", false);
                object.item = Some(Item::Fireball);
                object
            };
            objects.push(item);
        }
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
    use Ai::*;
    if let Some(ai) = objects[monster_id].ai.take() {
        let new_ai = match ai {
            Basic => ai_basic(monster_id, map, objects, fov_map, messages),
            Confused {
                previous_ai,
                num_turns,
            } => ai_confused(monster_id, map, objects, messages, previous_ai, num_turns),
        };
        objects[monster_id].ai = Some(new_ai);
    }
}

pub fn ai_basic(
    monster_id: usize, 
    map: &Map, 
    objects: &mut [Object], 
    fov_map: &FovMap,
    messages: &mut Messages,
    ) -> Ai {
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
    Ai::Basic
}

fn ai_confused(
    monster_id: usize,
    map: &Map,
    objects: &mut [Object],
    messages: &mut Messages,
    previous_ai: Box<Ai>,
    num_turns: i32,
) -> Ai {
    if num_turns >=0 {
        // Still confused
        // move in a random direction, and decrese number of turns remaining
        move_by(
            monster_id,
            rand::thread_rng().gen_range(-1, 2),
            rand::thread_rng().gen_range(-1, 2),
            map,
            objects,
        );
        Ai::Confused {
            previous_ai: previous_ai,
            num_turns: num_turns - 1,
        }
    } else {
        // restore previous_ai (delete this one)
        message(
            messages,
            format!("The {} is no longer confused!", objects[monster_id].name),
            colors::RED,
        );
        *previous_ai
    }
}

pub fn pick_item_up(
    object_id:  usize,
    objects:    &mut Vec<Object>,
    inventory:  &mut Vec<Object>,
    messages:   &mut Messages,
) {
    if inventory.len() >= 26 {
        message(
            messages,
            format!(
                "Your inventory is full, cannot pick up {}.",
                objects[object_id].name
            ),
            colors::RED,
        );
    } else {
        let item = objects.swap_remove(object_id);
        message(
            messages,
            format!("You picked up a {}!", item.name),
            colors::GREEN,
        );
        inventory.push(item);
    }
}

pub fn monster_death(monster: &mut Object, messages: &mut Messages) {
    message(
        messages, 
        format!("{} is dead!", monster.name), 
        colors::ORANGE,
    );
    monster.char = '%';
    monster.color = colors::DARK_RED;
    monster.blocks = false;
    monster.fighter = None;
    monster.ai = None;
    monster.name = format!("remains of {}", monster.name);
}

pub fn player_death(player: &mut Object, messages: &mut Messages) {
    // the game ended!
    message(
        messages,
        format!("You died!"),
        colors::RED,
        );

    //for added effect, transform the player into a corpse!
    player.char = '%';
    player.color = colors::DARK_RED;
}

pub fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) +1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

pub fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) +1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

pub fn is_blocked(x: i32, y: i32, map: &Map, objects: &[Object]) -> bool {
    //check for blocking tile
    if map[x as usize][y as usize].blocked {
        return true;
    }

    //check for blocking object
    objects.iter().any(|object| {
        object.blocks && object.pos() == (x, y)
    })
}


pub fn move_by(id: usize, dx: i32, dy: i32, map: &Map, objects: &mut [Object]) {
    let (x, y) = objects[id].pos();
    if !is_blocked(x + dx, y + dy, map, objects) {
        objects[id].set_pos(x + dx, y + dy);
    }
}

pub fn move_towards(id: usize, target_x: i32, target_y: i32, map: &Map, objects: &mut [Object]) {
    // vector from this object to the target, and distance
    let dx = target_x - objects[id].x;
    let dy = target_y - objects[id].y;
    let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

    // normalize it to length 1 (preserving direction), then round it and
    // convert to integer so the movement is restricted to the map grid
    let dx = (dx as f32 / distance).round() as i32;
    let dy = (dy as f32 / distance).round() as i32;
    move_by(id, dx, dy, map, objects);
}

pub fn mut_two<T>(first_index: usize, second_index: usize, items: &mut [T]) -> (&mut T, &mut T) {
    assert_ne!(first_index, second_index);
    let split_at_index = cmp::max(first_index, second_index);
    let (first_slice, second_slice) = items.split_at_mut(split_at_index);
    if first_index < second_index {
        (&mut first_slice[first_index], &mut second_slice[0])
    } else {
        (&mut second_slice[0], &mut first_slice[second_index])
    }
}


pub fn render_bar(
    panel: &mut Offscreen,
    x:              i32,
    y:              i32,
    total_width:    i32,
    name:           &str,
    value:          i32,
    maximum:        i32,
    bar_color:      Color,
    back_color:     Color,
    ) {
    // render a bar (HP, experience, etc.) First calculate the width of the bar
    let bar_width = (value as f32 / maximum as f32 * total_width as f32) as i32;

    // render the background first
    panel.set_default_background(back_color);
    panel.rect(x, y, total_width, 1, false, BackgroundFlag::Screen);

    // now render the bar on top
    panel.set_default_background(bar_color);
    if bar_width > 0 {
        panel.rect(x, y, bar_width, 1, false, BackgroundFlag::Screen);
    }

    //finally some centered text with values
    panel.set_default_foreground(colors::WHITE);
    panel.print_ex(x + total_width / 2, 
                   y, 
                   BackgroundFlag::None, 
                   TextAlignment::Center, 
                   &format!("{}: {}/{}", name, value, maximum),
                   );
}

pub fn message<T: Into<String>>(messages: &mut Messages, message: T, color: Color) {
    // if the buffer is full, remove the first message to make room for the new one
     if messages.len() == constants::MSG_HEIGHT {
         messages.remove(0);
     }
     // add the new line as a tuple, with the text and the color
     messages.push((message.into(), color));
}

fn menu<T: AsRef<str>>(header: &str, options: &[T], width: i32, root: &mut Root)
-> Option<usize> {
    //body
    assert!(
        options.len() <= 26,
        "Cannot have a menu with more thatn 26 options."
    );

    let header_height = root.get_height_rect(0, 0, width, constants::SCREEN_HEIGHT, header);
    let height = options.len() as i32 + header_height;

    let mut window = Offscreen::new(width, height);

    window.set_default_foreground(colors::WHITE);
    window.print_rect_ex(
        0,
        0,
        width,
        height,
        BackgroundFlag::None,
        TextAlignment::Left,
        header,
    );

    for (index, option_text) in options.iter().enumerate() {
        let menu_letter = (b'a' + index as u8) as char;
        let text = format!("({}) {}", menu_letter, option_text.as_ref());
        window.print_ex(
            0,
            header_height + index as i32,
            BackgroundFlag::None,
            TextAlignment::Left,
            text,
        );
    }
    let x = constants::SCREEN_WIDTH / 2 - width /2;
    let y = constants::SCREEN_HEIGHT / 2 - height /2;
    tcod::console::blit(&mut window, (0, 0), (width, height), root, (x, y), 1.0, 0.7);
    
    root.flush();
    let key = root.wait_for_keypress(true);

    if key.printable.is_alphabetic() {
        let index = key.printable.to_ascii_lowercase() as usize - 'a' as usize;
        if index < options.len() {
            Some(index)
        } else {
            None
        }
    } else {
        None
    }
}

fn inventory_menu(inventory: &[Object], header: &str, root: &mut Root)
-> Option<usize> {
    let options = if inventory.len() == 0 {
        vec!["Inventory is empty.".into()]
    } else {
        inventory.iter().map(|item| { item.name.clone() }).collect()
    };

    let inventory_index = menu(header, &options, constants::INVENTORY_WIDTH, root);
    
    if inventory.len() > 0 {
        inventory_index
    } else {
        None
    }
}

fn use_item(
    inventory_id: usize,
    inventory: &mut Vec<Object>,
    objects: &mut [Object],
    messages: &mut Messages,
    map:        &mut Map,
    tcod: &mut Tcod,
) {
    use Item::*;
    if let Some(item) = inventory[inventory_id].item {
        let on_use = match item {
            Heal => cast_heal,
            Lightning => cast_lightning,
            Confuse => cast_confuse,
            Fireball => cast_fireball,
        };
        match on_use(inventory_id, objects, messages, map, tcod) {
            UseResult::UsedUp => {
                // destroy after use, unless it was cancelled

                inventory.remove(inventory_id);
            }
            UseResult::Cancelled => {
                message(messages, "Cancelled", colors::WHITE);
            }
        }
    } else {
        message(
            messages,
        format!("The {} cannot be used.",
                inventory[inventory_id].name),
                colors::WHITE,
        );
    }
}

fn cast_heal(
    _inventory_id: usize,
    objects: &mut [Object],
    messages: &mut Messages,
    _map: &mut Map,
    _tcod: &mut Tcod,
    ) -> UseResult {
        // heal the player
        if let Some(fighter) = objects[PLAYER].fighter {
            if fighter.hp == fighter.max_hp {
                message(messages, "You are already at full health.",
                        colors::RED);
                return UseResult::Cancelled;
            }
            message(
                messages,
                "Your wounds start to feel better!",
                colors::LIGHT_VIOLET,
            );
            objects[PLAYER].heal(HEAL_AMOUNT);
            return UseResult::UsedUp;
        }
        UseResult::Cancelled
}

fn cast_lightning(
    _inventory_id: usize,
    objects: &mut [Object],
    messages: &mut Messages,
    _map: &mut Map,
    tcod: &mut Tcod,
) -> UseResult {
    // find closest enemy (inside a maximum range and damage it)
    let monster_id = closest_monster(LIGHTNING_RANGE, objects, tcod);
    if let Some(monster_id) = monster_id {
        // zap it
        message(
            messages,
            format!(
                "A lightning bolt strikes the {} with a loud clap! \
                The damage is {} hit points",
                objects[monster_id].name, LIGHTNING_DAMAGE
            ),
            colors::LIGHT_BLUE,
        );
        objects[monster_id].take_damage(LIGHTNING_DAMAGE, messages);
        UseResult::UsedUp
    } else {
        message(
            messages,
            "No enemy is close enough to strike.",
            colors::RED,
        );
        UseResult::Cancelled
    }
}

fn cast_confuse(
    _inventory_id: usize,
    objects: &mut [Object],
    messages: &mut Messages,
    map: &mut Map,
    tcod: &mut Tcod,
) -> UseResult {
    // ask the player for a target to confuse
    message(
        messages,
        "Left-click an enemy to confuse it, or right-click to cancel.",
        colors::LIGHT_CYAN,
    );
    let monster_id = target_monster(tcod, objects, map, messages, Some(CONFUSE_RANGE as f32));
    if let Some(monster_id) = monster_id {
        let old_ai = objects[monster_id].ai.take().unwrap_or(Ai::Basic);
        // replace the monster's AI with a "confused" one;
        // after some turns it will restore the old ai
        objects[monster_id].ai = Some(Ai::Confused {
            previous_ai: Box::new(old_ai),
            num_turns: CONFUSE_NUM_TURNS,
        });
        message(
            messages,
            format!(
                "The eyes of {} look vacant, as he starts to stumble around!",
                objects[monster_id].name
            ),
            colors::LIGHT_GREEN,
        );
        UseResult::UsedUp
    } else {
        //no enemy found within max range
        message(messages, "No enemy is close enough to strike.", colors::RED);
        UseResult::Cancelled
    }
}

fn cast_fireball(
    _inventory_id: usize,
    objects: &mut [Object],
    messages: &mut Messages,
    map: &mut Map,
    tcod: &mut Tcod,
) -> UseResult {
    // ask the player for a tile to throw a fireball at
    message(
        messages,
        "Left-click a target tile for the fireball, or right-click to cancel.",
        colors::LIGHT_CYAN,
    );
    let (x, y) = match target_tile(tcod, objects, map, messages, None) {
        Some(tile_pos) => tile_pos,
        None => return UseResult::Cancelled,
    };
    message(
        messages,
        format!(
            "The fireball explodes, burning everything within {} tiles!",
            FIREBALL_RADIUS
        ),
        colors::ORANGE,
    );

    for obj in objects {
        if obj.distance(x, y) <= FIREBALL_RADIUS as f32 && obj.fighter.is_some() {
            message(
                messages,
                format!(
                    "The {} gets burned for {} hit points.",
                    obj.name, FIREBALL_DAMAGE
                ),
                colors::ORANGE,
            );
            obj.take_damage(FIREBALL_DAMAGE, messages);
        }
    }

    UseResult::UsedUp
}

fn closest_monster(max_range: i32, objects: &mut [Object], tcod: &Tcod)
    -> Option<usize> {
        let mut closest_enemy = None;
        let mut closest_dist = (max_range + 1) as f32; //start with slightly more than max_range

        for (id, object) in objects.iter().enumerate() {
            if (id != PLAYER)
                && object.fighter.is_some()
                && object.ai.is_some()
                && tcod.fov.is_in_fov(object.x, object.y)
            {
                // calc distance between object and player
                let dist = objects[PLAYER].distance_to(object);
                if dist < closest_dist {
                    // it's closest, remember
                    closest_enemy = Some(id);
                    closest_dist = dist;
                }
            }
        }
        closest_enemy
}

fn target_tile(
    tcod: &mut Tcod,
    objects: &[Object],
    map: &mut Map,
    messages: &Messages,
    max_range: Option<f32>,
) -> Option<(i32, i32)> {
    use tcod::input::KeyCode::Escape;

    loop {
        // render the screen. This erases the inventory and shows the names
        // of objects under the mouse
        tcod.root.flush();
        let event = input::check_for_event(input::KEY_PRESS | input::MOUSE)
            .map(|e| e.1);

        let mut key = None;
        match event {
            Some(Event::Mouse(m)) => tcod.mouse = m,
            Some(Event::Key(k)) => key = Some(k),
            None => {}
        }
        render_all(tcod, objects, map, messages, false);

        let (x, y) = (tcod.mouse.cx as i32, tcod.mouse.cy as i32);

        let in_fov = (x < MAP_WIDTH) && (y < MAP_HEIGHT) && tcod.fov.is_in_fov(x, y);
        let in_range = max_range.map_or(true, |range| objects[PLAYER].distance(x, y) <= range);
        if tcod.mouse.lbutton_pressed && in_fov && in_range {
            return Some((x,y))
        }

        let escape = key.map_or(false, |k| k.code == Escape);
        if tcod.mouse.rbutton_pressed || escape {
            return None // cancel if the player right clicked or pressed Escape
        }
    }
}

fn target_monster(
    tcod: &mut Tcod,
    objects: &[Object],
    map: &mut Map,
    messages: &Messages,
    max_range: Option<f32>,
) -> Option<usize> {
    loop {
        match target_tile(tcod, objects, map, messages, max_range) {
            Some((x, y)) => {
                // return the first clicked monster, otherwise continue looping
                for (id, obj) in objects.iter().enumerate() {
                    if obj.pos() == (x, y) && obj.fighter.is_some() && id != PLAYER {
                        return Some(id);
                    }
                }
            }
            None => return None,
        }
    }
}

fn drop_item(
    inventory_id: usize,
    inventory: &mut Vec<Object>,
    objects: &mut Vec<Object>,
    messages: &mut Messages,
) {
    let mut item = inventory.remove(inventory_id);
    item.set_pos(objects[PLAYER].x, objects[PLAYER].y);
    message(
        messages,
        format!("You dropped a {}.", item.name),
        colors::YELLOW,
    );
    objects.push(item);
}
