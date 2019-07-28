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
    assets: Assets,
}

impl MainState{
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let canvas = graphics::Canvas::with_window_size(ctx)?;
        let font = graphics::Font::default();
        let text = graphics::Text::new(("Hello Rugue!", font, 24.0));
        let assets = Assets::new(ctx)?;
        Ok(MainState{ canvas, text, assets})
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
        let param = graphics::DrawParam::new()
            .dest(Point2::new(0.0, 0.0))
            .scale(nalgebra::Vector2::new(0.5, 0.5));
        graphics::draw(
            ctx,
            &self.assets.player_sprite,
            param,
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

struct Assets{
    player_sprite: graphics::Image,
    orc_sprite: graphics::Image,
    troll_sprite: graphics::Image,
    sword_sprite: graphics::Image,
    dagger_sprite: graphics::Image,
    shield_sprite: graphics::Image,
    potion_sprite: graphics::Image,
    scroll_sprite: graphics::Image,
    wall_sprite: graphics::Image,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let player_sprite = graphics::Image::new(ctx, "/player.png")?;
        let orc_sprite = graphics::Image::new(ctx, "/orc.png")?;
        let troll_sprite = graphics::Image::new(ctx, "/troll.png")?;
        let sword_sprite = graphics::Image::new(ctx, "/sword.png")?;
        let dagger_sprite = graphics::Image::new(ctx, "/dagger.png")?;
        let shield_sprite = graphics::Image::new(ctx, "/shield.png")?;
        let potion_sprite = graphics::Image::new(ctx, "/pot.png")?;
        let scroll_sprite = graphics::Image::new(ctx, "/scroll.png")?;
        let wall_sprite = graphics::Image::new(ctx, "/wall.png")?;

        Ok(Assets {
            player_sprite,
            orc_sprite,
            troll_sprite,
            sword_sprite,
            dagger_sprite,
            shield_sprite,
            potion_sprite,
            scroll_sprite,
            wall_sprite,
        })
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
