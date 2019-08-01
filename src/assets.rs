use ggez::graphics::*;
use ggez::Context;
use ggez::GameResult;

use crate::object::*;

pub struct Assets {
    pub player_sprite: Image,
    pub orc_sprite: Image,
    pub troll_sprite: Image,
    pub sword_sprite: Image,
    pub dagger_sprite: Image,
    pub shield_sprite: Image,
    pub potion_sprite: Image,
    pub scroll_sprite: Image,
    pub wall_sprite: Image,
    pub stairs_sprite: Image,
    pub corpse_sprite: Image,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let player_sprite = Image::new(ctx, "/player.png")?;
        let orc_sprite = Image::new(ctx, "/orc.png")?;
        let troll_sprite = Image::new(ctx, "/troll.png")?;
        let sword_sprite = Image::new(ctx, "/sword.png")?;
        let dagger_sprite = Image::new(ctx, "/dagger.png")?;
        let shield_sprite = Image::new(ctx, "/shield.png")?;
        let potion_sprite = Image::new(ctx, "/pot.png")?;
        let scroll_sprite = Image::new(ctx, "/scroll.png")?;
        let wall_sprite = Image::new(ctx, "/wall.png")?;
        let stairs_sprite = Image::new(ctx, "/stairs.png")?;
        let corpse_sprite = Image::new(ctx, "/corpse.png")?;

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
            stairs_sprite,
            corpse_sprite,
        })
    }

    pub fn object_image(&mut self, object: &Object) -> &mut Image {
        match object.object_type {
            ObjectType::Player => {
                if !object.alive {
                    &mut self.corpse_sprite
                } else {
                    &mut self.player_sprite
                }
            }
            ObjectType::Orc => {
                if !object.alive {
                    &mut self.corpse_sprite
                } else {
                    &mut self.orc_sprite
                }
            }
            ObjectType::Troll => {
                if !object.alive {
                    &mut self.corpse_sprite
                } else {
                    &mut self.troll_sprite
                }
            }
            ObjectType::ItemSword => &mut self.sword_sprite,
            ObjectType::ItemDagger => &mut self.dagger_sprite,
            ObjectType::ItemScroll => &mut self.scroll_sprite,
            ObjectType::ItemShield => &mut self.shield_sprite,
            ObjectType::ItemPotion => &mut self.potion_sprite,
            ObjectType::Stairs => &mut self.stairs_sprite,
        }
    }
}
