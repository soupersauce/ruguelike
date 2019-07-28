use ggez::{self, Context, graphics, event::EventHandler, GameResult};
use ggez::graphics::Color;
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
}

impl EventHandler for GameplayState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        graphics::present(ctx);
        Ok(())
    }
}

impl GameplayState {
    pub fn new(ctx: &mut Context) -> GameResult<GameplayState> {
        let log = vec![];
        let assets = Assets::new(ctx)?;
        let mut player = Object::new(0, 0, assets.player_sprite, "player", true);
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
        let map = Map::new(&mut objects, dungeon_level, assets);
        let inventory = vec![];
        Ok(GameplayState{ canvas, assets, map, log, inventory, dungeon_level})
    }
}
