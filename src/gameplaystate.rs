use std::cmp;

use rand::Rng;

use ggez::{self, Context, graphics, event::EventHandler, GameResult};
use ggez::graphics::Color;
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
}

impl EventHandler for GameplayState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        self.draw_map(ctx);
        self.draw_objects(ctx);

        graphics::present(ctx)?;
        Ok(())
    }

}

impl GameplayState {
    pub fn new(ctx: &mut Context) -> GameResult<GameplayState> {
        let log = vec![];
        let assets = Assets::new(ctx)?;
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
        map.initialize_fov(ctx);
        let inventory = vec![];
        Ok(GameplayState{ canvas, assets, map, log, inventory, dungeon_level, objects})
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

    fn draw_map(&mut self, ctx: &mut Context) {
        for o in &self.objects {
            let sprite = self.assets.object_image(&o);
            let params = graphics::DrawParam::default()
                .dest(map_to_window_coords(o.x, o.y))
                .scale(core::Vector2::new(0.5, 0.5));
            
            graphics::draw(ctx, sprite, params);
        }
    }

    pub fn handle_keys(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) -> PlayerAction {
        use PlayerAction::*;
        let player_alive = self.objects[PLAYER].alive;
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
                self.player_move_or_attack(0, -1);
                TookTurn
            }

            (KeyCode::Down, true) | (KeyCode::Numpad2, true) => {
                self.player_move_or_attack(0, 1);
                TookTurn
            }

            (KeyCode::Left, true) | (KeyCode::Numpad4, true) => {
                self.player_move_or_attack(-1, 0);
                TookTurn
            }

            (KeyCode::Right, true) | (KeyCode::Numpad6, true) => {
                self.player_move_or_attack(1, 0);
                TookTurn
            }

            (KeyCode::Home, true) | (KeyCode::Numpad7, true) => {
                self.player_move_or_attack(-1, -1);
                TookTurn
            }

            (KeyCode::PageUp, true) | (KeyCode::Numpad9, true) => {
                self.player_move_or_attack(1, -1);
                TookTurn
            }

            (KeyCode::End, true) | (KeyCode::Numpad1, true) => {
                self.player_move_or_attack(-1, 1);
                TookTurn
            }

            (KeyCode::PageDown, true) | (KeyCode::Numpad3, true) => {
                self.player_move_or_attack(1, 1);
                TookTurn
            }

            (KeyCode::Numpad5, true) => TookTurn,
            (KeyCode::G, true) => {
                //pick up item
                let item_id = self.objects
                    .iter()
                    .position(|object| object.pos() == self.objects[PLAYER].pos() && object.item.is_some());
                if let Some(item_id) = item_id {
                    self.pick_item_up(item_id);
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
            //         drop_item(inventory_index, game, selfobjects);
            //     }
            //     DidntTakeTurn
            // }

            (KeyCode::Period, true) => {
                // go down stairs, if the player is on them
                let player_on_stairs = self.objects
                    .iter()
                    .any(|object| object.pos() == self.objects[PLAYER].pos() && object.name == "stairs");
                if player_on_stairs {
                    // next_level(tcod, objects, game);
                }
                DidntTakeTurn
            }

            (KeyCode::C, true) => {
                let player = self.objects[PLAYER];
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
                        player.max_hp(self),
                        player.power(self),
                        player.defense(self),
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

    fn player_move_or_attack(&mut self, dx: i32, dy: i32) {
        //coordinates player is moving to or attacking
        let x = self.objects[PLAYER].x + dx;
        let y = self.objects[PLAYER].y + dy;

        // try to find an attackable object
        let target_id = self.objects
            .iter()
            .position(|object| object.fighter.is_some() && object.pos() == (x, y));

        //attack if target found, move otherwise
        match target_id {
            Some(target_id) => {
                let (player, target) = mut_two(PLAYER, target_id, &mut self.objects);
                player.attack(target, self);
            }
            None => {
                self.move_by(PLAYER, dx, dy);
            }
        }
    }

    fn level_up(&mut self, game: &mut GameplayState) {
        let player = &mut self.objects[PLAYER];
        let level_up_xp = LEVEL_UP_BASE + player.level * LEVEL_UP_FACTOR;

        if player.fighter.as_ref().map_or(0, |f| f.xp) >= level_up_xp {
            player.level += 1;
            self.log.add(
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
        &mut self,
        monster_id: usize,
    ) {
        use crate::object::Ai::*;
        if let Some(ai) = self.objects[monster_id].ai.take() {
            let new_ai = match ai {
                Basic => self.ai_basic(monster_id),
                Confused {
                    previous_ai,
                    num_turns,
                } => self.ai_confused(monster_id, previous_ai, num_turns),
            };
            self.objects[monster_id].ai = Some(new_ai);
        }
    }

    pub fn ai_basic(
        &mut self,
        monster_id: usize,
    ) -> Ai {
        // a basic monster takes its turn. If you can see it, it can see you
        let (monster_x, monster_y) = self.objects[monster_id].pos();
        if self.map.fov_map.is_in_fov(monster_x as usize, monster_y as usize) {
            if self.objects[monster_id].distance_to(&self.objects[PLAYER]) >= 2.0 {
                // move towards player if far away
                let (player_x, player_y) = self.objects[PLAYER].pos();
                self.move_towards(monster_id, player_x, player_y);
            } else if self.objects[PLAYER].fighter.map_or(false, |f| f.hp > 0) {
                // close enough to attack! (if the player is still alive)
                let (monster, player) = mut_two(monster_id, PLAYER, &mut self.objects);
                monster.attack(player, self);
            }
        }
        Ai::Basic
    }

    fn ai_confused(
        &mut self,
        monster_id: usize,
        previous_ai: Box<Ai>,
        num_turns: i32,
    ) -> Ai {
        if num_turns >= 0 {
            // Still confused
            // move in a random direction, and decrese number of turns remaining
            self.move_by(
                monster_id,
                rand::thread_rng().gen_range(-1, 2),
                rand::thread_rng().gen_range(-1, 2),
            );
            Ai::Confused {
                previous_ai: previous_ai,
                num_turns: num_turns - 1,
            }
        } else {
            // restore previous_ai (delete this one)
            self.log.add(
                format!("The {} is no longer confused!", self.objects[monster_id].name),
                RED,
            );
            *previous_ai
        }
    }

    fn pick_item_up(&mut self, object_id: usize) {
        if self.inventory.len() >= 26 {
            self.log.add(
                format!(
                    "Your inventory is full, cannot pick up {}.",
                    self.objects[object_id].name
                ),
                RED,
            );
        } else {
            let item = self.objects.swap_remove(object_id);
            self.log
                .add(format!("You picked up a {}!", item.name), GREEN);
            let index = self.inventory.len();
            let slot = item.equipment.map(|e| e.slot);
            self.inventory.push(item);

            if let Some(slot) = slot {
                if get_equipped_in_slot(slot, &self.inventory).is_none() {
                    self.inventory[index].equip(&mut self.log);
                }
            }
        }
    }

    fn move_by(&mut self, id: usize, dx: i32, dy: i32) {
        let (x, y) = self.objects[id].pos();
        if !self.is_blocked(x + dx, y + dy) {
            self.objects[id].set_pos(x + dx, y + dy);
        }
    }

    fn move_towards(&mut self, id: usize, target_x: i32, target_y: i32) {
        // vector from this object to the target, and distance
        let dx = target_x - self.objects[id].x;
        let dy = target_y - self.objects[id].y;
        let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

        // normalize it to length 1 (preserving direction), then round it and
        // convert to integer so the movement is restricted to the map grid
        let dx = (dx as f32 / distance).round() as i32;
        let dy = (dy as f32 / distance).round() as i32;
        self.move_by(id, dx, dy);
    }

    pub fn is_blocked(&mut self, x: i32, y: i32) -> bool {
        //check for blocking tile
        if self.map.map_grid[x as usize][y as usize].blocked {
            return true;
        }

        //check for blocking object
        self.objects
            .iter()
            .any(|object| object.blocks && object.pos() == (x, y))
    }

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

