use ggez::{self, Context, graphics, event::EventHandler, GameResult};
use ggez::graphics::Color;
use ggez::nalgebra::{core, geometry};

use crate::object::*;
use crate::assets::Assets;
use crate::map::Map;

pub type Messages = Vec<(String, Color)>;

pub trait MessageLog {
    fn add<T: Into<String>>(&mut self, message: T, color: Color);
}

impl MessageLog for Vec<(String, Color)> {
    fn add<T: Into<String>>(&mut self, message: T, color: Color) {
        self.push((message.into(), color));
    }
}

pub struct GameplayState {
    canvas: graphics::Canvas,
    assets: Assets,
    map: Map,
    pub log: Messages,
    pub inventory: Vec<Object>,
    dungeon_level: u32,
    objects: Vec<Object>,
}

impl EventHandler for GameplayState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        for o in &self.objects {
            let sprite = self.assets.object_image(&o);
            let params = graphics::DrawParam::default()
                .dest(map_to_window_coords(o.x, o.y))
                .scale(core::Vector2::new(0.5, 0.5));
            
            graphics::draw(ctx, sprite, params)?;
        }
        graphics::present(ctx)?;
        Ok(())
    }
}

impl GameplayState {
    pub fn new(ctx: &mut Context) -> GameResult<GameplayState> {
        let log = vec![];
        let assets = Assets::new(ctx)?;
        let mut player = Object::new(0, 0, ObjectType::Player, "player", true);
        let dungeon_level = 1;
        player.alive = true;
        player.fighter = Some(Fighter {
            base_max_hp: 100,
            hp: 100,
            base_defense: 1,
            base_power: 3,
            on_death: DeathCallback::Player,
            xp: 0,
        });

        let mut objects = vec![player];
        let canvas = graphics::Canvas::with_window_size(ctx)?;
        let map = Map::new(&mut objects, dungeon_level);
        let inventory = vec![];
        Ok(GameplayState{ canvas, assets, map, log, inventory, dungeon_level, objects})
    }
}

fn map_to_window_coords(x: i32, y: i32) -> geometry::Point2<f32> {
    let xn = x*16;
    let yn = y*16;
    geometry::Point2::new(xn as f32, yn as f32)
}

