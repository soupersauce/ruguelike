use tcod::colors::{self};
use tcod::console::*;
use tcod::map::Map as FovMap;
use tcod::input::{self, Event};

mod lib;

use crate::lib::*;

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

    let mut inventory = vec![];

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
        let player_action = handle_keys(key, &mut root, &map, &mut objects, &mut messages, &mut inventory);

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

