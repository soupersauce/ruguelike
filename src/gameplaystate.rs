use std::cmp;

use rand::Rng;

use ggez::{self, Context, event::EventHandler, GameResult, timer};
use ggez::graphics::{self, Color, Image, spritebatch, DrawParam};
use ggez::input::*;
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::nalgebra::{core, geometry};

use crate::object::*;
use crate::assets::Assets;
use crate::map::Map;
use crate::constants::*;

pub type Messages = Vec<(String, Color)>;

pub trait MessageLog {
    fn add<T: Into<String>>(&mut self, message: T, color: Color);
}

impl MessageLog for Vec<(String, Color)> {
    fn add<T: Into<String>>(&mut self, message: T, color: Color) {
        self.push((message.into(), color));
    }
}

pub struct GameplayState {
    canvas: graphics::Canvas,
    assets: Assets,
    map: Map,
    pub log: Messages,
    pub inventory: Vec<Object>,
    dungeon_level: u32,
    objects: Vec<Object>,
    spritebatch: spritebatch::SpriteBatch,
}

impl EventHandler for GameplayState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if timer::ticks(ctx) % 100 == 0 {
            println!("Delta frame time: {:?}", timer::delta(ctx));
            println!("Average FPS: {:?}", timer::fps(ctx));
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        self.draw_map_sb(ctx);
        self.draw_objects_sb(ctx);

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        handle_keys(ctx, keycode, _keymods, _repeat, &mut self.objects, &mut self.inventory, &mut self.log, &self.map);
    }

}

impl GameplayState {
    pub fn new(ctx: &mut Context) -> GameResult<GameplayState> {
        let log = vec![];
        let assets = Assets::new(ctx)?;
        let sprite_sheet = graphics::Image::new(ctx, "/sheet.png")?;
        let spritebatch = graphics::spritebatch::SpriteBatch::new(sprite_sheet);
        let mut player = Object::new(0, 0, ObjectType::Player, "player", true);
        let dungeon_level = 1;
        player.alive = true;
        player.fighter = Some(Fighter {
            base_max_hp: 100,
            hp: 100,
            base_defense: 1,
            base_power: 3,
            on_death: DeathCallback::Player,
            xp: 0,
        });

        let mut objects = vec![player];
        let canvas = graphics::Canvas::with_window_size(ctx)?;
        let mut map = Map::new(&mut objects, dungeon_level);
        //map.initialize_fov(ctx);
        let inventory = vec![];
        Ok(GameplayState{ canvas, assets, map, log, inventory, dungeon_level, objects, spritebatch})
    }

    fn draw_objects(&mut self, ctx: &mut Context) {
        for o in &self.objects {
            let sprite = self.assets.object_image(&o);
            let params = graphics::DrawParam::default()
                .dest(map_to_window_coords(o.x, o.y))
                .scale(core::Vector2::new(0.5, 0.5));
            
            graphics::draw(ctx, sprite, params);
        }
    }

    fn draw_objects_sb(&mut self, ctx: &mut Context) {
        let spritewidth = 32;
        let corpse = 0;
        let dagger = 1;
        let orc = 2;
        let player = 3;
        let potion = 4;
        let scroll = 5;
        let shield = 6;
        let stairs = 7;
        let sword = 8;
        let troll = 9;
        let _wall = 10;

        for o in &self.objects {
            match o.object_type {
                    ObjectType::Player => {
                        if !o.alive {
                            let p = DrawParam::default()
                                .src(rect_from_sprite_offset(corpse, spritewidth))
                                .dest(map_to_window_coords(o.x, o.y))
                                .scale(core::Vector2::new(0.5, 0.5));

                                    self.spritebatch.add(p);
                        } else {
                            let p = DrawParam::default()
                                .src(rect_from_sprite_offset(player, spritewidth))
                                .dest(map_to_window_coords(o.x, o.y))
                                .scale(core::Vector2::new(0.5, 0.5));

                                    self.spritebatch.add(p);
                                }
                    },
                    ObjectType::Orc => {
                        if !o.alive {
                            let p = DrawParam::default()
                                .src(rect_from_sprite_offset(corpse, spritewidth))
                                .dest(map_to_window_coords(o.x, o.y))
                                .scale(core::Vector2::new(0.5, 0.5));

                                    self.spritebatch.add(p);
                        } else {
                            let p = DrawParam::default()
                                .src(rect_from_sprite_offset(orc, spritewidth))
                                .dest(map_to_window_coords(o.x, o.y))
                                .scale(core::Vector2::new(0.5, 0.5));

                                    self.spritebatch.add(p);
                        } 
                    },
                    ObjectType::Troll => {
                        if !o.alive {
                            let p = DrawParam::default()
                                .src(rect_from_sprite_offset(corpse, spritewidth))
                                .dest(map_to_window_coords(o.x, o.y))
                                .scale(core::Vector2::new(0.5, 0.5));

                                    self.spritebatch.add(p);
                        } else {
                            let p = DrawParam::default()
                                .src(rect_from_sprite_offset(troll, spritewidth))
                                .dest(map_to_window_coords(o.x, o.y))
                                .scale(core::Vector2::new(0.5, 0.5));

                                    self.spritebatch.add(p);
                        }
                    },
                    ObjectType::Stairs => {
                                let p = DrawParam::default()
                                    .src(rect_from_sprite_offset(stairs, spritewidth))
                                    .dest(map_to_window_coords(o.x, o.y))
                                    .scale(core::Vector2::new(0.5, 0.5));

                                    self.spritebatch.add(p);
                            },
                    ObjectType::ItemSword => {
                                let p = DrawParam::default()
                                    .src(rect_from_sprite_offset(sword, spritewidth))
                                    .dest(map_to_window_coords(o.x, o.y))
                                    .scale(core::Vector2::new(0.5, 0.5));

                                    self.spritebatch.add(p);
                            },
                    ObjectType::ItemDagger => {
                                let p = DrawParam::default()
                                    .src(rect_from_sprite_offset(dagger, spritewidth))
                                    .dest(map_to_window_coords(o.x, o.y))
                                    .scale(core::Vector2::new(0.5, 0.5));

                                    self.spritebatch.add(p);
                            },
                    ObjectType::ItemPotion => {
                                let p = DrawParam::default()
                                    .src(rect_from_sprite_offset(potion, spritewidth))
                                    .dest(map_to_window_coords(o.x, o.y))
                                    .scale(core::Vector2::new(0.5, 0.5));

                                    self.spritebatch.add(p);
                            },
                    ObjectType::ItemScroll => {
                                let p = DrawParam::default()
                                    .src(rect_from_sprite_offset(scroll, spritewidth))
                                    .dest(map_to_window_coords(o.x, o.y))
                                    .scale(core::Vector2::new(0.5, 0.5));

                                    self.spritebatch.add(p);
                            },
                    ObjectType::ItemShield => {
                                let p = DrawParam::default()
                                    .src(rect_from_sprite_offset(shield, spritewidth))
                                    .dest(map_to_window_coords(o.x, o.y))
                                    .scale(core::Vector2::new(0.5, 0.5));

                                    self.spritebatch.add(p);
                            },
                    };
                }
        graphics::draw(ctx, &self.spritebatch, DrawParam::default());
        self.spritebatch.clear();
    }

    fn draw_map(&mut self, ctx: &mut Context) {
        let sprite = &self.assets.wall_sprite;
        for x in 0..MAP_WIDTH {
            for y in 0..MAP_HEIGHT {

            if self.map.map_grid[x as usize][y as usize].block_sight {
                let params = graphics::DrawParam::default()
                    .dest(map_to_window_coords(x, y))
                    .scale(core::Vector2::new(0.5, 0.5));
                graphics::draw(ctx, sprite, params);
            }}
        }
    }

    fn draw_map_sb(&mut self, ctx: &mut Context) {
        let sprite = &self.assets.wall_sprite;
        for x in 0..MAP_WIDTH {
            for y in 0..MAP_HEIGHT {

                if self.map.map_grid[x as usize][y as usize].block_sight {
                    let p = graphics::DrawParam::default()
                        .src(rect_from_sprite_offset(10, 32))
                        .dest(map_to_window_coords(x, y))
                        .scale(core::Vector2::new(0.5, 0.5));
                    self.spritebatch.add(p);
                }
            }
        }
        graphics::draw(ctx, &self.spritebatch, DrawParam::default());
        self.spritebatch.clear();
    }
}

fn rect_from_sprite_offset(offset: i32, sprite_dim: i32) -> graphics::Rect {
    graphics::Rect::new(offset as f32 * 0.090909090909090909090909090909, 0.0, 0.090909090909090909090909090909, 1.0)
}

pub fn handle_keys(
    ctx: &mut Context,
    keycode: KeyCode,
    _keymod: KeyMods,
    _repeat: bool,
    objects: &mut Vec<Object>,
    inventory: &mut Vec<Object>,
    log: &mut Messages,
    map: &Map,
) -> PlayerAction {
    use PlayerAction::*;
    let player_alive = objects[PLAYER].alive;
    match (keycode, player_alive) {
        // (
        //     Key {
        //         code: Enter,
        //         alt: true,
        //         ..
        //     },
        //     _,
        // ) => {
        //     // alt+enter to toggle fullscreen
        //     let fullscreen = tcod.root.is_fullscreen();
        //     tcod.root.set_fullscreen(!fullscreen);
        //     DidntTakeTurn
        // }
        // (KeyCode::Escape, .. }, _) => event::quit(ctx), //exit game

        (KeyCode::Up, true) | (KeyCode::Numpad8, true) => {
            player_move_or_attack(0, -1, objects, inventory, log, map);
            TookTurn
        }

        (KeyCode::Down, true) | (KeyCode::Numpad2, true) => {
            player_move_or_attack(0, 1, objects, inventory, log, map);
            TookTurn
        }

        (KeyCode::Left, true) | (KeyCode::Numpad4, true) => {
            player_move_or_attack(-1, 0, objects, inventory, log, map);
            TookTurn
        }

        (KeyCode::Right, true) | (KeyCode::Numpad6, true) => {
            player_move_or_attack(1, 0, objects, inventory, log, map);
            TookTurn
        }

        (KeyCode::Home, true) | (KeyCode::Numpad7, true) => {
            player_move_or_attack(-1, -1, objects, inventory, log, map);
            TookTurn
        }

        (KeyCode::PageUp, true) | (KeyCode::Numpad9, true) => {
            player_move_or_attack(1, -1, objects, inventory, log, map);
            TookTurn
        }

        (KeyCode::End, true) | (KeyCode::Numpad1, true) => {
            player_move_or_attack(-1, 1, objects, inventory, log, map);
            TookTurn
        }

        (KeyCode::PageDown, true) | (KeyCode::Numpad3, true) => {
            player_move_or_attack(1, 1, objects, inventory, log, map);
            TookTurn
        }

        (KeyCode::Numpad5, true) => TookTurn,
        (KeyCode::G, true) => {
            //pick up item
            let item_id = objects
                .iter()
                .position(|object| object.pos() == objects[PLAYER].pos() && object.item.is_some());
            if let Some(item_id) = item_id {
                &mut pick_item_up(item_id, objects, log, inventory);
            }
            DidntTakeTurn
        }

        // (KeyCode::I, true) => {
        //     // show the inventory
        //     let inventory_index = inventory_menu(
        //         &game.inventory,
        //         "Press the key next to an item to use it, or any other to cancel. \n",
        //         &mut tcod.root,
        //     );

        //     if let Some(inventory_index) = inventory_index {
        //         use_item(inventory_index, objects, game, tcod);
        //     }
        //     DidntTakeTurn
        // }
        // (KeyCode::D, true) => {
        //     // show the inventory
        //     let inventory_index = inventory_menu(
        //         &game.inventory,
        //         "Press the key next to an item to drop it, or any other to cancel. \n",
        //         &mut tcod.root,
        //     );

        //     if let Some(inventory_index) = inventory_index {
        //         drop_item(inventory_index, game, objects);
        //     }
        //     DidntTakeTurn
        // }

        (KeyCode::Period, true) => {
            // go down stairs, if the player is on them
            let player_on_stairs = objects
                .iter()
                .any(|object| object.pos() == objects[PLAYER].pos() && object.name == "stairs");
            if player_on_stairs {
                // next_level(tcod, objects, game);
            }
            DidntTakeTurn
        }

        (KeyCode::C, true) => {
            let player = &objects[PLAYER];
            let level = player.level;
            let level_up_xp = LEVEL_UP_BASE + player.level * LEVEL_UP_FACTOR;
            if let Some(fighter) = player.fighter.as_ref() {
                let msg = format!(
                    "Character information

                    Level: {}
                    Experience: {}
                    Experience to next level: {}

                    Max HP: {}
                    Atk: {}
                    Def: {}",
                    level,
                    fighter.xp,
                    level_up_xp,
                    player.max_hp(inventory),
                    player.power(inventory),
                    player.defense(inventory),
                );
                // msgbox(&msg, CHARACTER_SCREEN_WIDTH, &mut tcod.root);
            }
            DidntTakeTurn
        }

        _ => DidntTakeTurn,
    }
}

// pub fn drop_item(inventory_id: usize, game: &mut GameplayState, objects: &mut Vec<Object>) {
//     let mut item = game.inventory.remove(inventory_id);
//     if item.equipment.is_some() {
//         item.unequip(&mut game.log);
//     }
//     item.set_pos(objects[PLAYER].x, objects[PLAYER].y);
//     game.log
//         .add(format!("You dropped a {}.", item.name), colors::YELLOW);
//     objects.push(item);
// }

fn player_move_or_attack(dx: i32, dy: i32, objects: &mut [Object], inventory: &Vec<Object>, log: &mut Messages, map: &Map) {
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
            player.attack(target, inventory, log);
        }
        None => {
            move_by(PLAYER, dx, dy, &map, objects);
        }
    }
}

fn level_up(objects: &mut [Object], game: &mut GameplayState) {
    let player = &mut objects[PLAYER];
    let level_up_xp = LEVEL_UP_BASE + player.level * LEVEL_UP_FACTOR;

    if player.fighter.as_ref().map_or(0, |f| f.xp) >= level_up_xp {
        player.level += 1;
        game.log.add(
            format!(
                "Your battle skills grow stronger! You reached level {}!",
                player.level
            ),
            YELLOW,
        );
        // let fighter = player.fighter.as_mut().unwrap();
        // let mut choice = None;
        // while choice.is_none() {
        //     choice = menu(
        //         "Level up! Choose a stat raise: \n",
        //         &[
        //             format!("Constitution (+20 HP, from {})", fighter.base_max_hp),
        //             format!("Strength (+1 atk, from {})", fighter.base_power),
        //             format!("Agility (+1 def, from {})", fighter.base_defense),
        //         ],
        //         LEVEL_SCREEN_WIDTH,
        //         &mut tcod.root,
        //     );
        // }
        // fighter.xp -= level_up_xp;
        // match choice.unwrap() {
        //     0 => {
        //         fighter.base_max_hp += 20;
        //         fighter.hp += 20;
        //     }
        //     1 => {
        //         fighter.base_power += 1;
        //     }
        //     2 => {
        //         fighter.base_defense += 1;
        //     }
        //     _ => unreachable!(),
        // }
    }
}

pub fn ai_take_turn(
    monster_id: usize,
    inventory: &mut Vec<Object>,
    objects: &mut [Object],
    map: &Map,
    log: &mut Messages,
) {
    use crate::object::Ai::*;
    if let Some(ai) = objects[monster_id].ai.take() {
        let new_ai = match ai {
            Basic => ai_basic(monster_id, inventory, objects, map, log),
            Confused {
                previous_ai,
                num_turns,
            } => ai_confused(monster_id, objects, inventory, previous_ai, num_turns, map, log),
        };
        objects[monster_id].ai = Some(new_ai);
    }
}

pub fn ai_basic(
    monster_id: usize,
    inventory: &Vec<Object>,
    objects: &mut [Object],
    map: &Map,
    log: &mut Messages,
) -> Ai {
    // a basic monster takes its turn. If you can see it, it can see you
    let (monster_x, monster_y) = objects[monster_id].pos();
    if map.fov_map.is_in_fov(monster_x as usize, monster_y as usize) {
        if objects[monster_id].distance_to(&objects[PLAYER]) >= 2.0 {
            // move towards player if far away
            let (player_x, player_y) = objects[PLAYER].pos();
            move_towards(monster_id, player_x, player_y, &map, objects);
        } else if objects[PLAYER].fighter.map_or(false, |f| f.hp > 0) {
            // close enough to attack! (if the player is still alive)
            let (monster, player) = mut_two(monster_id, PLAYER, objects);
            monster.attack(player, inventory, log);
        }
    }
    Ai::Basic
}

fn ai_confused(
    monster_id: usize,
    objects: &mut [Object],
    inventory: &mut Vec<Object>,
    previous_ai: Box<Ai>,
    num_turns: i32,
    map: &Map,
    log: &mut Messages,
) -> Ai {
    if num_turns >= 0 {
        // Still confused
        // move in a random direction, and decrese number of turns remaining
        move_by(
            monster_id,
            rand::thread_rng().gen_range(-1, 2),
            rand::thread_rng().gen_range(-1, 2),
            &map,
            objects,
        );
        Ai::Confused {
            previous_ai: previous_ai,
            num_turns: num_turns - 1,
        }
    } else {
        // restore previous_ai (delete this one)
        log.add(
            format!("The {} is no longer confused!", objects[monster_id].name),
            RED,
        );
        *previous_ai
    }
}

fn pick_item_up(object_id: usize, objects: &mut Vec<Object>, log: &mut Messages, inventory: &mut Vec<Object> ) {
    if inventory.len() >= 26 {
        log.add(
            format!(
                "Your inventory is full, cannot pick up {}.",
                objects[object_id].name
            ),
            RED,
        );
    } else {
        let item = objects.swap_remove(object_id);
        log
            .add(format!("You picked up a {}!", item.name), GREEN);
        let index = inventory.len();
        let slot = item.equipment.map(|e| e.slot);
        inventory.push(item);

        if let Some(slot) = slot {
            if get_equipped_in_slot(slot, &inventory).is_none() {
                inventory[index].equip(log);
            }
        }
    }
}

fn move_by(id: usize, dx: i32, dy: i32, map: &Map, objects: &mut [Object]) {
    let (x, y) = objects[id].pos();
    if !is_blocked(x + dx, y + dy, map, objects) {
        objects[id].set_pos(x + dx, y + dy);
    }
}

fn move_towards(id: usize, target_x: i32, target_y: i32, map: &Map, objects: &mut [Object]) {
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

pub fn is_blocked(x: i32, y: i32, map: &Map, objects: &mut [Object]) -> bool {
    //check for blocking tile
    if map.map_grid[x as usize][y as usize].blocked {
        return true;
    }

    //check for blocking object
    objects
        .iter()
        .any(|object| object.blocks && object.pos() == (x, y))
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

pub fn get_equipped_in_slot(slot: Slot, inventory: &[Object]) -> Option<usize> {
    for (inventory_id, item) in inventory.iter().enumerate() {
        if item
            .equipment
            .as_ref()
            .map_or(false, |e| e.equipped && e.slot == slot)
        {
            return Some(inventory_id);
        }
    }
    None
}


fn map_to_window_coords(x: i32, y: i32) -> geometry::Point2<f32> {
    let xn = x*16;
    let yn = y*16;
    geometry::Point2::new(xn as f32, yn as f32)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerAction {
    TookTurn,
    DidntTakeTurn,
    Exit,
}

