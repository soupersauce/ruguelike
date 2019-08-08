use std::borrow::BorrowMut;
// use tcod::colors::{self, Color};
use ggez::graphics::Color;
use ggez::graphics::{self, *};
use ggez::Context;

use crate::constants::*;
use crate::gameplaystate::*;

#[derive(Debug)]
pub struct Object {
    pub x: i32,
    pub y: i32,
    pub object_type: ObjectType,
    // pub color: Color,
    pub name: String,
    pub blocks: bool,
    pub alive: bool,
    pub fighter: Option<Fighter>,
    pub ai: Option<Ai>,
    pub item: Option<Item>,
    pub always_visible: bool,
    pub level: i32,
    pub equipment: Option<Equipment>,
}

impl Object {
    pub fn new(x: i32, y: i32, object_type: ObjectType, name: &str, blocks: bool) -> Self {
        Object {
            x,
            y,
            object_type,
            // color,
            name: name.into(),
            blocks,
            alive: false,
            fighter: None,
            ai: None,
            item: None,
            always_visible: false,
            level: 1,
            equipment: None,
        }
    }

    // /// set the color and then draw the character that represents this object at its position
    // pub fn draw(&self, ctx: &mut Context) {
    //     con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    // }

    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn distance_to(&self, other: &Object) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }

    pub fn distance(&self, x: i32, y: i32) -> f32 {
        (((x - self.x).pow(2) + (y - self.y).pow(2)) as f32).sqrt()
    }

    pub fn take_damage(&mut self, damage: i32, log: &mut Messages) -> Option<i32> {
        //apply damage if possible
        if let Some(fighter) = self.fighter.as_mut() {
            if damage > 0 {
                fighter.hp -= damage;
            }
        }
        // check for death, call the death function
        if let Some(fighter) = self.fighter {
            if fighter.hp <= 0 {
                self.alive = false;
                fighter.on_death.callback(self, log);
                return Some(fighter.xp);
            }
        }
        None
    }

    pub fn attack(&mut self, target: &mut Object, inventory: &[Object], log: &mut Messages) {
        // a simple formula for attack damage
        let damage = self.power(inventory) - target.defense(inventory);
        if damage > 0 {
            //make the target take some damage
            log.add(
                format!(
                    "{} attacks {} for {} hit points.",
                    self.name, target.name, damage
                ),
                WHITE,
            );
            if let Some(xp) = target.take_damage(damage, log) {
                self.fighter.as_mut().unwrap().xp += xp;
            }
        } else {
            log.add(
                format!(
                    "{} attacks {} but it has no effect!",
                    self.name, target.name
                ),
                WHITE,
            );
        }
    }

    pub fn heal(&mut self, amount: i32, inventory: &[Object]) {
        let max_hp = self.max_hp(inventory);
        if let Some(ref mut fighter) = self.fighter {
            fighter.hp += amount;
            if fighter.hp > max_hp {
                fighter.hp = max_hp;
            }
        }
    }

    /// Equip object and show a message about it
    pub fn equip(&mut self, log: &mut Messages) {
        if self.item.is_none() {
            log.add(
                format!("Can't equip {:?} because it's not an item.", self),
                RED,
            );
            return;
        };

        if let Some(ref mut equipment) = self.equipment {
            if !equipment.equipped {
                equipment.equipped = true;
                log.add(
                    format!("Equipped {} on {}.", self.name, equipment.slot),
                    LIGHT_GREEN,
                );
            }
        } else {
            log.add(
                format!("Can't equip {:?} because it's not an Equipment.", self),
                RED,
            );
        }
    }
    /// unequip object and show a message about it
    pub fn unequip(&mut self, log: &mut Vec<(String, Color)>) {
        if self.item.is_none() {
            log.add(
                format!("Can't unequip {:?} because it's not an item.", self),
                RED,
            );
            return;
        };

        if let Some(ref mut equipment) = self.equipment {
            if equipment.equipped {
                equipment.equipped = false;
                log.add(
                    format!("Unequipped {} on {}.", self.name, equipment.slot),
                    LIGHT_YELLOW,
                );
            }
        } else {
            log.add(
                format!("Can't unquip {:?} because it's not an Equipment.", self),
                RED,
            );
        }
    }

    pub fn power(&self, inventory: &[Object]) -> i32 {
        let base_power = self.fighter.map_or(0, |f| f.base_power);
        let bonus: i32 = self
            .get_all_equipped(inventory)
            .iter()
            .map(|e| e.power_bonus)
            .sum();

        base_power + bonus
    }

    pub fn defense(&self, inventory: &[Object]) -> i32 {
        let base_defense = self.fighter.map_or(0, |f| f.base_defense);
        let bonus: i32 = self
            .get_all_equipped(inventory)
            .iter()
            .map(|e| e.defense_bonus)
            .sum();

        base_defense + bonus
    }

    pub fn max_hp(&self, inventory: &[Object]) -> i32 {
        let base_max_hp = self.fighter.map_or(0, |f| f.base_max_hp);
        let bonus: i32 = self
            .get_all_equipped(inventory)
            .iter()
            .map(|e| e.max_hp_bonus)
            .sum();

        base_max_hp + bonus
    }

    pub fn get_all_equipped(&self, inventory: &[Object]) -> Vec<Equipment> {
        if self.name == "player" {
            inventory
                .iter()
                .filter(|item| item.equipment.map_or(false, |e| e.equipped))
                .map(|item| item.equipment.unwrap())
                .collect()
        } else {
            vec![] //other objects have no equipment
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fighter {
    pub base_max_hp: i32,
    pub hp: i32,
    pub base_defense: i32,
    pub base_power: i32,
    pub on_death: DeathCallback,
    pub xp: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum DeathCallback {
    Player,
    Monster,
}

impl DeathCallback {
    fn callback(self, object: &mut Object, log: &mut Messages) {
        use DeathCallback::*;
        let callback: fn(&mut Object, &mut Messages) = match self {
            Player => player_death,
            Monster => monster_death,
        };
        callback(object, log);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
/// An object that can be equipped, yielding bonuses.
pub struct Equipment {
    pub slot: Slot,
    pub equipped: bool,
    pub power_bonus: i32,
    pub defense_bonus: i32,
    pub max_hp_bonus: i32,
}

impl Equipment {}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Slot {
    LeftHand,
    RightHand,
    Head,
}

impl std::fmt::Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Slot::LeftHand => write!(f, "left hand"),
            Slot::RightHand => write!(f, "right hand"),
            Slot::Head => write!(f, "head"),
        }
    }
}

pub fn monster_death(monster: &mut Object, log: &mut Messages) {
    log.add(
        format!(
            "{} is dead! +{} xp",
            monster.name,
            monster.fighter.unwrap().xp
        ),
        ORANGE,
    );
    monster.alive = false;
    monster.blocks = false;
    monster.fighter = None;
    monster.ai = None;
    monster.name = format!("remains of {}", monster.name);
}

pub fn player_death(player: &mut Object, log: &mut Messages) {
    // the game ended!
    log.add("You died!".to_string(), RED);

    //for added effect, transform the player into a corpse!
    player.alive = false;
}

#[derive(Debug)]
pub enum ObjectType {
    Player,
    Orc,
    Troll,
    ItemScroll,
    ItemSword,
    ItemDagger,
    ItemShield,
    ItemPotion,
    Stairs,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Ai {
    Basic,
    Confused {
        previous_ai: Box<Ai>,
        num_turns: i32,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Item {
    Heal,
    Lightning,
    Confuse,
    Fireball,
    Sword,
    Shield,
}

pub enum UseResult {
    UsedUp,
    Cancelled,
    UsedAndKept,
}
