use doryen_fov::MapData as FovMap;
use rand::distributions::{IndependentSample, Weighted, WeightedChoice};
use rand::Rng;
use std::cmp;

use ggez::graphics::{self, *};
use ggez::Context;

use crate::constants::*;
use crate::object::*;

pub type MapGrid = Vec<Vec<Tile>>;

pub struct Map {
    pub map_grid: MapGrid,
    pub fov_map: FovMap,
}

impl Map {
    pub fn new(objects: &mut Vec<Object>, level: u32) -> Map {
        // fill map with "unblocked" tiles
        let mut map_grid = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

        assert_eq!(&objects[PLAYER] as *const _, &objects[0] as *const _);
        objects.truncate(1);

        let mut rooms = vec![];

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
                create_room(new_room, &mut map_grid);

                place_objects(new_room, objects, &map_grid, level);

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
                        create_h_tunnel(prev_x, new_x, prev_y, &mut map_grid);
                        create_v_tunnel(prev_y, new_y, new_x, &mut map_grid);
                    } else {
                        // first vertically, then horizontally
                        create_v_tunnel(prev_y, new_y, prev_x, &mut map_grid);
                        create_h_tunnel(prev_x, new_x, new_y, &mut map_grid);
                    }
                }
                rooms.push(new_room);
            }
        }
        let (last_room_x, last_room_y) = rooms[rooms.len() - 1].center();
        let mut stairs = Object::new(
            last_room_x,
            last_room_y,
            ObjectType::Stairs,
            "stairs",
            false,
        );
        stairs.always_visible = true;
        objects.push(stairs);
        let fov_map = FovMap::new(MAP_HEIGHT as usize, MAP_HEIGHT as usize);
        Map { map_grid, fov_map }
    }

    pub fn initialize_fov(&mut self, ctx: &mut Context) {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                if !self.map_grid[x as usize][y as usize].block_sight {
                    &mut self.fov_map.set_transparent(x as usize, y as usize, false);
                } else {
                    &mut self.fov_map.set_transparent(x as usize, y as usize, true);
                }
            }
        }
        graphics::clear(ctx, BLACK);
    }
}
pub struct Transition {
    level: u32,
    value: u32,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
    pub explored: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
            explored: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
            explored: false,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }

    pub fn intersects_with(&self, other: &Rect) -> bool {
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }
}

fn place_objects(room: Rect, objects: &mut Vec<Object>, map: &MapGrid, level: u32) {
    // choose random number of monsters
    let max_monsters = from_dungeon_level(
        &[
            Transition { level: 1, value: 2 },
            Transition { level: 4, value: 3 },
            Transition { level: 6, value: 5 },
        ],
        level,
    );

    let num_monsters = rand::thread_rng().gen_range(0, max_monsters + 1);
    let troll_chance = from_dungeon_level(
        &[
            Transition {
                level: 3,
                value: 15,
            },
            Transition {
                level: 5,
                value: 30,
            },
            Transition {
                level: 7,
                value: 60,
            },
        ],
        level,
    );

    let monster_chances = &mut [
        Weighted {
            weight: 80,
            item: "orc",
        },
        Weighted {
            weight: troll_chance,
            item: "troll",
        },
    ];

    let monster_choice = WeightedChoice::new(monster_chances);

    for _ in 0..num_monsters {
        // chose random spot for this monster
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

        let mut monster = match monster_choice.ind_sample(&mut rand::thread_rng()) {
            "orc" => {
                let mut orc = Object::new(x, y, ObjectType::Orc, "Orc", true);
                orc.fighter = Some(Fighter {
                    base_max_hp: 20,
                    hp: 20,
                    base_defense: 0,
                    base_power: 4,
                    on_death: DeathCallback::Monster,
                    xp: 35,
                });
                orc.ai = Some(Ai::Basic);
                orc
            }
            "troll" => {
                let mut troll = Object::new(x, y, ObjectType::Troll, "Troll", true);
                troll.fighter = Some(Fighter {
                    base_max_hp: 30,
                    hp: 30,
                    base_defense: 2,
                    base_power: 8,
                    on_death: DeathCallback::Monster,
                    xp: 100,
                });
                troll.ai = Some(Ai::Basic);
                troll
            }
            _ => unreachable!(),
        };
        monster.alive = true;
        objects.push(monster);
    }

    let max_items = from_dungeon_level(
        &[
            Transition { level: 1, value: 1 },
            Transition { level: 1, value: 1 },
        ],
        level,
    );

    let item_chances = &mut [
        Weighted {
            weight: 35,
            item: Item::Heal,
        },
        Weighted {
            weight: from_dungeon_level(
                &[Transition {
                    level: 4,
                    value: 25,
                }],
                level,
            ),
            item: Item::Lightning,
        },
        Weighted {
            weight: from_dungeon_level(
                &[Transition {
                    level: 6,
                    value: 25,
                }],
                level,
            ),
            item: Item::Fireball,
        },
        Weighted {
            weight: from_dungeon_level(
                &[Transition {
                    level: 2,
                    value: 10,
                }],
                level,
            ),
            item: Item::Confuse,
        },
        Weighted {
            weight: from_dungeon_level(&[Transition { level: 4, value: 5 }], level),
            item: Item::Sword,
        },
        Weighted {
            weight: from_dungeon_level(
                &[Transition {
                    level: 8,
                    value: 15,
                }],
                level,
            ),
            item: Item::Shield,
        },
    ];

    let item_choice = WeightedChoice::new(item_chances);

    let num_items = rand::thread_rng().gen_range(0, max_items + 1);

    for _ in 0..num_items {
        // choose random spot for this item
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

        // only place it if the tile is not blocked
        if !is_blocked(x, y, &map, objects) {
            let mut item = match item_choice.ind_sample(&mut rand::thread_rng()) {
                Item::Heal => {
                    // create a healing object
                    let mut object =
                        Object::new(x, y, ObjectType::ItemPotion, "Healing potion", false);
                    object.item = Some(Item::Heal);
                    object
                }
                Item::Lightning => {
                    let mut object = Object::new(
                        x,
                        y,
                        ObjectType::ItemScroll,
                        "Scroll of lightning bolt",
                        false,
                    );
                    object.item = Some(Item::Lightning);
                    object
                }
                Item::Fireball => {
                    let mut object =
                        Object::new(x, y, ObjectType::ItemScroll, "Scroll of fireball", false);
                    object.item = Some(Item::Fireball);
                    object
                }
                Item::Confuse => {
                    let mut object =
                        Object::new(x, y, ObjectType::ItemScroll, "Scroll of confusion", false);
                    object.item = Some(Item::Confuse);
                    object
                }
                Item::Sword => {
                    // Create a sword
                    let mut object = Object::new(x, y, ObjectType::ItemSword, "sword", false);
                    object.item = Some(Item::Sword);
                    object.equipment = Some(Equipment {
                        equipped: false,
                        slot: Slot::RightHand,
                        power_bonus: 3,
                        defense_bonus: 0,
                        max_hp_bonus: 0,
                    });
                    object
                }
                Item::Shield => {
                    // Create a shield
                    let mut object = Object::new(x, y, ObjectType::ItemShield, "shield", false);
                    object.item = Some(Item::Shield);
                    object.equipment = Some(Equipment {
                        equipped: false,
                        slot: Slot::LeftHand,
                        power_bonus: 3,
                        defense_bonus: 1,
                        max_hp_bonus: 0,
                    });
                    object
                }
            };
            item.always_visible = true;
            objects.push(item);
        }
    }
}

fn create_room(room: Rect, map: &mut MapGrid) {
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut MapGrid) {
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut MapGrid) {
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn from_dungeon_level(table: &[Transition], level: u32) -> u32 {
    table
        .iter()
        .rev()
        .find(|transition| level >= transition.level)
        .map_or(0, |transition| transition.value)
}

pub fn is_blocked(x: i32, y: i32, map: &MapGrid, objects: &[Object]) -> bool {
    //check for blocking tile
    if map[x as usize][y as usize].blocked {
        return true;
    }

    //check for blocking object
    objects
        .iter()
        .any(|object| object.blocks && object.pos() == (x, y))
}
