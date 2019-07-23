#[macro_use]
extern crate serde_derive;

use tcod::console::*;
use tcod::map::Map as FovMap;

use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};

mod game;
use crate::game::*;

struct Game {

}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        // Load/Create resources such as images here.
        Game {
            // TODO
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Code here
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        //draw code here
        graphics::present(ctx)
    }
}

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("Rugue", "sauceCo")
        .build()
        .expect("Error: Could not create context");

    let mut game = Game::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occurred: {}", e),
    }
}
