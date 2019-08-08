#![allow(dead_code, unused_imports)]
#[macro_use]
extern crate serde_derive;

// use tcod::console::*;
// use tcod::map::Map as FovMap;

use std::env;
// use std::io::{Read, Write};
use std::path;
// use std::str;

use ggez;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, *};
use ggez::nalgebra;
use ggez::{conf::*, Context, ContextBuilder, GameResult};

mod assets;
mod constants;
mod gameplaystate;
mod map;
mod object;

use crate::gameplaystate::GameplayState;

fn main() -> GameResult {
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
        resizable: false,
    };
    let window_setup = WindowSetup {
        title: "Rugue".to_owned(),
        samples: NumSamples::Zero,
        vsync: true,
        icon: "".to_owned(),
        srgb: true,
    };

    let mut cb = ContextBuilder::new("Rugue", "sauceCo")
        .window_mode(window_mode)
        .window_setup(window_setup);

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        cb = cb.add_resource_path(path);
    }

    let (ctx, event_loop) = &mut cb.build()?;

    graphics::set_drawable_size(ctx, 1280.0, 720.0)?;

    let game = &mut GameplayState::new(ctx)?;

    // Run!
    event::run(ctx, event_loop, game)
}
