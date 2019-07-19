use tcod::colors::{self, Color};

use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub struct Object {
    pub x:              i32,
    pub y:              i32,
    pub char:           char,
    pub color:          Color,
    pub name:           String,
    pub blocks:         bool,
    pub alive:          bool,
    pub fighter:        Option<Fighter>,
    pub ai:             Option<Ai>,
    pub item:           Option<Item>,
    pub always_visible: bool,
    pub level:          i32,
    pub equipment:      Option<Equipment>,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color, name: &str, blocks: bool) -> Self {
        Object { 
            x, 
            y, 
            char, 
            color, 
            name:           name.into(),
            blocks,
            alive:          false,
            fighter:        None,
            ai:             None,
            item:           None,
            always_visible: false,
            level:          1,
            equipment:      None,
        }
    }


    /// set the color and then draw the character that represents this object at its position
    pub fn draw(&self, con: &mut Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn distance_to(&self, other: &Object) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }

    pub fn distance(&self, x: i32, y: i32) -> f32 {
        (((x - self.x).pow(2) + (y - self.y).pow(2)) as f32).sqrt()
    }

    pub fn take_damage(&mut self, damage: i32, game: &mut Game) 
    -> Option<i32> {
        //apply damage if possible
        if let Some(fighter) = self.fighter.as_mut() {
            if damage > 0 {
                fighter.hp -= damage;
            }
        }
        // check for death, call the death function
        if let Some(fighter) = self.fighter {
            if fighter.hp <= 0 {
                self.alive = false;
                fighter.on_death.callback(self, game);
                return Some(fighter.xp);
            }
        }
        None
    }

    pub fn attack(&mut self, target: &mut Object, game: &mut Game) {
        // a simple formula for attack damage
        let damage = self.power(game) - target.defense(game);
        if damage > 0 {
            //make the target take some damage
            game.log.add(
                format!("{} attacks {} for {} hit points.",self.name, target.name, damage),
                colors::WHITE,
            );
            if let Some(xp) = target.take_damage(damage, game) {
                self.fighter.as_mut().unwrap().xp += xp;
            }
        } else {
            game.log.add(
                format!("{} attacks {} but it has no effect!", self.name, target.name),
                colors::WHITE,
            );
        }
    }

    pub fn heal(&mut self, amount: i32, game: &Game) {
        let max_hp = self.max_hp(game);
        if let Some(ref mut fighter) = self.fighter {
            fighter.hp += amount;
            if fighter.hp > max_hp {
                fighter.hp = max_hp;
            }
        }
    }

    /// Equip object and show a message about it
    pub fn equip(&mut self, log: &mut Vec<(String, Color)>) {
        if self.item.is_none() {
            log.add(
                format!("Can't equip {:?} because it's not an item.", self),
                colors::RED,
            );
            return;
        };

        if let Some(ref mut equipment) = self.equipment {
            if !equipment.equipped {
                equipment.equipped = true;
                log.add(
                    format!("Equipped {} on {}.", self.name, equipment.slot),
                    colors::LIGHT_GREEN,
                );
            }
        } else {
            log.add(
                format!("Can't equip {:?} because it's not an Equipment.", self),
                       colors::RED,
            );
        }
    }
    /// unequip object and show a message about it
    pub fn unequip(&mut self, log: &mut Vec<(String, Color)>) {
        if self.item.is_none() {
            log.add(
                format!("Can't unequip {:?} because it's not an item.", self),
                colors::RED,
            );
            return;
        };

        if let Some(ref mut equipment) = self.equipment {
            if equipment.equipped {
                equipment.equipped = false;
                log.add(
                    format!("Unequipped {} on {}.", self.name, equipment.slot),
                    colors::LIGHT_YELLOW,
                );
            }
        } else {
            log.add(
                format!("Can't unquip {:?} because it's not an Equipment.", self),
                       colors::RED,
            );
        }
    }

    pub fn power(&self, game: &Game) -> i32 {
        let base_power = self.fighter.map_or(0, |f| f.base_power);
        let bonus: i32 = self
            .get_all_equipped(game)
            .iter()
            .map(|e| e.power_bonus)
            .sum();
            
        base_power + bonus
    }

    pub fn defense(&self, game: &Game) -> i32 {
        let base_defense = self.fighter.map_or(0, |f| f.base_defense);
        let bonus: i32 = self
            .get_all_equipped(game)
            .iter()
            .map(|e| e.defense_bonus)
            .sum();
            
        base_defense + bonus
    }

    pub fn max_hp(&self, game: &Game) -> i32 {
        let base_max_hp = self.fighter.map_or(0, |f| f.base_max_hp);
        let bonus: i32 = self
            .get_all_equipped(game)
            .iter()
            .map(|e| e.max_hp_bonus)
            .sum();
            
        base_max_hp + bonus
    }

    pub fn get_all_equipped(&self, game: &Game) -> Vec<Equipment> {
        if self.name == "player" {
            game.inventory
                .iter()
                .filter(|item| item.equipment.map_or(false, |e| e.equipped))
                .map(|item| item.equipment.unwrap())
                .collect()
        } else {
            vec![] //other objects have no equipment
        }
    }

}

