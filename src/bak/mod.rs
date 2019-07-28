use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

use tcod::colors::{self, Color};
use tcod::console::*;
use tcod::input::Mouse;
use tcod::map::Map as FovMap;

pub mod constants;
pub mod functions;
pub mod object;

pub use crate::game::constants::*;
pub use crate::game::functions::*;
pub use crate::game::object::*;

pub struct Tcod {
    pub root: Root,
    pub con: Offscreen,
    pub panel: Offscreen,
    pub fov: FovMap,
    pub mouse: Mouse,
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub map: Map,
    pub log: Messages,
    pub inventory: Vec<Object>,
    pub dungeon_level: u32,
}

pub struct Transition {
    level: u32,
    value: u32,
}

pub type Map = Vec<Vec<Tile>>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerAction {
    TookTurn,
    DidntTakeTurn,
    Exit,
}

