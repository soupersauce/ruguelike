#[macro_use]
extern crate serde_derive;

use tcod::console::*;
use tcod::map::Map as FovMap;

use ggez::{graphics, Context, ContextBuilder, GameResult, conf::*};
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
    let window_mode = WindowMode {
        width: 1280.0,
        height: 720.0,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
        resizable: true,
    };
    let window_setup = WindowSetup {
        title: "Rugue".to_owned(),
        samples: NumSamples::Zero,
        vsync: true,
        icon: "".to_owned(),
        srgb: true,
    };

    let (mut ctx, mut event_loop) = ContextBuilder::new("Rugue", "sauceCo")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .build()
        .expect("Error: Could not create context");

    let mut game = Game::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occurred: {}", e),
    }
}
