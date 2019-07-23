#[macro_use]
extern crate serde_derive;

use tcod::console::*;
use tcod::map::Map as FovMap;

use ggez::{self, *};

mod game;
use crate::game::*;

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("Rugue", "sauceCo")
        .build()
        .expect("Error: Could not create context");

    let game = Game::new(&mut ctx);
}
