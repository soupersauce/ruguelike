use std::cmp;
use tcod::console::*;
use tcod::colors::{self, Color};

pub mod constants;
pub mod functions;

// use crate::lib::functions::*;
pub use crate::lib::constants::*;
pub use crate::lib::functions::*;

#[derive(Debug)]
pub struct Object {
    pub x:      i32,
    pub y:      i32,
    pub char:       char,
    pub color:      Color,
    pub name:   String,
    pub blocks:     bool,
    pub alive:  bool,
    pub fighter:    Option<Fighter>,
    pub ai:         Option<Ai>,
    pub item:       Option<Item>,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color, name: &str, blocks: bool) -> Self {
        Object { 
            x, 
            y, 
            char, 
            color, 
            name:       name.into(),
            blocks,
            alive:      false,
            fighter:    None,
            ai:         None,
            item:       None,
        }
    }


    /// set the color and then draw the character that represents this object at its position
    pub fn draw(&self, con: &mut Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

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

    pub fn take_damage(&mut self, damage: i32, messages: &mut Messages) {
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
                fighter.on_death.callback(self, messages);
            }
        }
    }

    pub fn attack(&mut self, target: &mut Object, messages: &mut Messages) {
        // a simple formula for attack damage
        let damage = self.fighter.map_or(0, |f| f.power) - target.fighter.map_or(0, |f| f.defense);
        if damage > 0 {
            //make the target take some damage
            message(
                messages,
                format!("{} attacks {} for {} hit points.",self.name, target.name, damage),
                colors::WHITE,
            );
            target.take_damage(damage, messages);
        } else {
            message(
                messages,
                format!("{} attacks {} but it has no effect!", self.name, target.name),
                colors::WHITE,
            );
        }
    }

    pub fn heal(&mut self, amount: i32) {
        if let Some(ref mut fighter) = self.fighter {
            fighter.hp += amount;
            if fighter.hp > fighter.max_hp {
                fighter.hp = fighter.max_hp;
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
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

pub type Map = Vec<Vec<Tile>>;

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Rect {
    pub fn new( x: i32, y: i32, w: i32, h: i32 ) -> Self {
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

pub fn create_room(room: Rect, map: &mut Map) {
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 +1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerAction {
    TookTurn,
    DidntTakeTurn,
    Exit,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fighter {
    pub max_hp:     i32,
    pub hp:         i32,
    pub defense:    i32,
    pub power:      i32,
    pub on_death:   DeathCallback,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeathCallback {
    Player,
    Monster,
}

impl DeathCallback {
    fn callback(self, object: &mut Object, messages: &mut Messages) {
        use DeathCallback::*;
        let callback: fn(&mut Object, &mut Messages) = match self {
            Player => player_death,
            Monster => monster_death,
        };
        callback(object, messages);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ai;

pub type Messages = Vec<(String, Color)>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Item {
    Heal,
}

pub enum UseResult {
    UsedUp,
    Cancelled,
}
