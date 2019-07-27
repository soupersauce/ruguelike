#[macro_use]
extern crate serde_derive;

use tcod::console::*;
use tcod::map::Map as FovMap;

use std::env;
use std::io::{Read, Write};
use std::path;
use std::str;

use ggez;
use ggez::nalgebra;
use ggez::{Context, ContextBuilder, GameResult, conf::*, filesystem};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, *};
use ggez::graphics::spritebatch::*;
use aseprite;
use serde_json;

mod game;
use crate::game::*;

type Point2 = nalgebra::geometry::Point2<f32>;
type Vector2 = nalgebra::base::Vector2<f32>;

struct MainState{
    canvas: graphics::Canvas,
    text: graphics::Text,
    spritesheet: SpriteBatch,
}

impl MainState{
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let canvas = graphics::Canvas::with_window_size(ctx)?;
        let font = graphics::Font::default();
        let text = graphics::Text::new(("Hello Rugue!", font, 24.0));
        // let file = filesystem::open(ctx, "/player.png")?;
        let sheetfile = Image::new(ctx, "/sheet.png")?;
        let mut spritesheet = SpriteBatch::new(sheetfile);
        Ok(MainState{ canvas, text, spritesheet})
    }
}

impl EventHandler for MainState{
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::set_canvas(ctx, Some(&self.canvas));
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        let (window_width, window_height) = graphics::size(ctx);
        graphics::draw(
            ctx,
            &self.spritesheet,
            (Point2::new(0.0, 0.0), graphics::WHITE),
        )?;

        // let window_size = graphics::size(ctx);
        // let scale = Vector2::new(
        //     0.5 * window_size.0 as f32 / self.canvas.image().width() as f32,
        //     0.5 * window_size.1 as f32 / self.canvas.image().width() as f32,
        // );

        graphics::set_canvas(ctx, None);
        graphics::clear(ctx, Color::new(0.0, 0.0, 0.0, 1.0));
        graphics::draw(ctx, &self.canvas, DrawParam::default()
                       .dest(Point2::new(0.0, 0.0))
                       // .scale(scale),
                       )?;
        // graphics::draw(ctx, &self.canvas, DrawParam::default()
        //                .dest(Point2::new(400.0, 300.0))
        //                .scale(scale),
        //                )?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let new_rect = graphics::Rect::new(0.0, 0.0, width, height);
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();
    }
}

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

    let game = &mut MainState::new(ctx)?;

    // Run!
    event::run(ctx, event_loop, game) 
}
