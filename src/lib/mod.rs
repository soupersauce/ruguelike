use std::cmp;
use tcod::colors::*;
use tcod::console::*;
use tcod::colors::{self, Color};

pub mod constants;

#[derive(Debug)]
pub struct Object {
    pub x:      i32,
    pub y:      i32,
    pub char:       char,
    pub color:      Color,
    pub name:   String,
    pub blocks:     bool,
    pub alive:  bool,
    pub fighter:    Option<Fighter>,
    pub ai:         Option<Ai>,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color, name: &str, blocks: bool) -> Self {
        Object { 
            x, 
            y, 
            char, 
            color, 
            name:       name.into(),
            blocks,
            alive:      false,
            fighter:    None,
            ai:         None,
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

    pub fn take_damage(&mut self, damage: i32, messages: &mut Messages) {
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
                fighter.on_death.callback(self, messages);
            }
        }
    }

    pub fn attack(&mut self, target: &mut Object, messages: &mut Messages) {
        // a simple formula for attack damage
        let damage = self.fighter.map_or(0, |f| f.power) - target.fighter.map_or(0, |f| f.defense);
        if damage > 0 {
            //make the target take some damage
            message(
                messages,
                format!("{} attacks {} for {} hit points.",self.name, target.name, damage),
                colors::WHITE,
            );
            target.take_damage(damage, messages);
        } else {
            message(
                messages,
                format!("{} attacks {} but it has no effect!", self.name, target.name),
                colors::WHITE,
            );
        }
    }


}

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
    pub explored: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile { 
            blocked: false, 
            block_sight: false,
            explored: false,
        }
    }

    pub fn wall() -> Self {
        Tile { 
            blocked: true, 
            block_sight: true,
            explored: false,
        }
    }
}

pub type Map = Vec<Vec<Tile>>;

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Rect {
    pub fn new( x: i32, y: i32, w: i32, h: i32 ) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }

    pub fn intersects_with(&self, other: &Rect) -> bool {
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }
}

pub fn create_room(room: Rect, map: &mut Map) {
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 +1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerAction {
    TookTurn,
    DidntTakeTurn,
    Exit,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fighter {
    pub max_hp:     i32,
    pub hp:         i32,
    pub defense:    i32,
    pub power:      i32,
    pub on_death:   DeathCallback,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeathCallback {
    Player,
    Monster,
}

impl DeathCallback {
    fn callback(self, object: &mut Object, messages: &mut Messages) {
        use DeathCallback::*;
        let callback: fn(&mut Object, &mut Messages) = match self {
            Player => player_death,
            Monster => monster_death,
        };
        callback(object, messages);
    }
}

pub fn monster_death(monster: &mut Object, messages: &mut Messages) {
    message(
        messages, 
        format!("{} is dead!", monster.name), 
        colors::ORANGE,
    );
    monster.char = '%';
    monster.color = colors::DARK_RED;
    monster.blocks = false;
    monster.fighter = None;
    monster.ai = None;
    monster.name = format!("remains of {}", monster.name);
}

pub fn player_death(player: &mut Object, messages: &mut Messages) {
    // the game ended!
    message(
        messages,
        format!("You died!"),
        colors::RED,
        );

    //for added effect, transform the player into a corpse!
    player.char = '%';
    player.color = colors::DARK_RED;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ai;

pub type Messages = Vec<(String, Color)>;

pub fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) +1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

pub fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) +1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

pub fn is_blocked(x: i32, y: i32, map: &Map, objects: &[Object]) -> bool {
    //check for blocking tile
    if map[x as usize][y as usize].blocked {
        return true;
    }

    //check for blocking object
    objects.iter().any(|object| {
        object.blocks && object.pos() == (x, y)
    })
}


pub fn move_by(id: usize, dx: i32, dy: i32, map: &Map, objects: &mut [Object]) {
    let (x, y) = objects[id].pos();
    if !is_blocked(x + dx, y + dy, map, objects) {
        objects[id].set_pos(x + dx, y + dy);
    }
}

pub fn move_towards(id: usize, target_x: i32, target_y: i32, map: &Map, objects: &mut [Object]) {
    // vector from this object to the target, and distance
    let dx = target_x - objects[id].x;
    let dy = target_y - objects[id].y;
    let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

    // normalize it to length 1 (preserving direction), then round it and
    // convert to integer so the movement is restricted to the map grid
    let dx = (dx as f32 / distance).round() as i32;
    let dy = (dy as f32 / distance).round() as i32;
    move_by(id, dx, dy, map, objects);
}

pub fn mut_two<T>(first_index: usize, second_index: usize, items: &mut [T]) -> (&mut T, &mut T) {
    assert_ne!(first_index, second_index);
    let split_at_index = cmp::max(first_index, second_index);
    let (first_slice, second_slice) = items.split_at_mut(split_at_index);
    if first_index < second_index {
        (&mut first_slice[first_index], &mut second_slice[0])
    } else {
        (&mut second_slice[0], &mut first_slice[second_index])
    }
}


pub fn render_bar(
    panel: &mut Offscreen,
    x:              i32,
    y:              i32,
    total_width:    i32,
    name:           &str,
    value:          i32,
    maximum:        i32,
    bar_color:      Color,
    back_color:     Color,
    ) {
    // render a bar (HP, experience, etc.) First calculate the width of the bar
    let bar_width = (value as f32 / maximum as f32 * total_width as f32) as i32;

    // render the background first
    panel.set_default_background(back_color);
    panel.rect(x, y, total_width, 1, false, BackgroundFlag::Screen);

    // now render the bar on top
    panel.set_default_background(bar_color);
    if bar_width > 0 {
        panel.rect(x, y, bar_width, 1, false, BackgroundFlag::Screen);
    }

    //finally some centered text with values
    panel.set_default_foreground(colors::WHITE);
    panel.print_ex(x + total_width / 2, 
                   y, 
                   BackgroundFlag::None, 
                   TextAlignment::Center, 
                   &format!("{}: {}/{}", name, value, maximum),
                   );
}
pub fn message<T: Into<String>>(messages: &mut Messages, message: T, color: Color) {
    // if the buffer is full, remove the first message to make room for the new one
     if messages.len() == constants::MSG_HEIGHT {
         messages.remove(0);
     }
     // add the new line as a tuple, with the text and the color
     messages.push((message.into(), color));
}

