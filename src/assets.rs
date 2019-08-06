use ggez::graphics::*;
use ggez::Context;
use ggez::GameResult;
use ggez::graphics::spritebatch::*;

use crate::object::*;

const NUM_OBJECT_SPRITES: i32  = 10;
const NUM_WALL_SPRITES: i32 = 2;
const SPRITE_WIDTH: i32 = 32;

pub struct Assets {
    pub object_sheet: SpriteBatch,
    pub wall_sheet: SpriteBatch,
    pub num_object_sprites: i32,
    pub num_wall_sprites: i32,
    pub sprite_width: i32,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let object_sheet = Image::new(ctx, "/sheet.png")?;
        let object_sheet = SpriteBatch::new(object_sheet);
        let num_object_sprites = NUM_OBJECT_SPRITES;
        let wall_sheet = Image::new(ctx, "/wall.png")?;
        let wall_sheet = SpriteBatch::new(wall_sheet);
        let num_wall_sprites = NUM_WALL_SPRITES;
        let sprite_width = SPRITE_WIDTH;
        Ok(Assets {
            object_sheet,
            wall_sheet,
            num_object_sprites,
            num_wall_sprites,
            sprite_width,
        })
    }

    pub fn object_sprite_width(&self) -> f32 {
        let total_width = self.sprite_width as f32 * self.num_object_sprites as f32;
        self.sprite_width as f32 / total_width
    }

    pub fn wall_sprite_width(&self) -> f32 {
        let total_width = self.sprite_width as f32 * self.num_wall_sprites as f32;
        self.sprite_width as f32 / total_width
    }
}
