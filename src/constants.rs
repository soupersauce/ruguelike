use doryen_fov::FovAlgorithm;
use ggez::graphics::Color;

pub static LIMIT_FPS: i32 = 60;

pub static SCREEN_WIDTH: i32 = 80;
pub static SCREEN_HEIGHT: i32 = 50;

pub static BAR_WIDTH: i32 = 20;
pub static PANEL_HEIGHT: i32 = 7;
pub static PANEL_Y: i32 = SCREEN_HEIGHT - PANEL_HEIGHT;

pub static MSG_X: i32 = BAR_WIDTH + 2;
pub static MSG_WIDTH: i32 = SCREEN_WIDTH - BAR_WIDTH - 2;
pub static MSG_HEIGHT: usize = PANEL_HEIGHT as usize - 1;

pub static MAP_WIDTH: i32 = 80;
pub static MAP_HEIGHT: i32 = 45;

pub const COLOR_DARK_WALL: Color = Color {r: 0.0, g: 0.0, b: 0.39, a: 1.0};
pub static COLOR_LIGHT_WALL: Color = Color {
    r: 0.50,
    g: 0.43,
    b: 0.19,
    a: 1.0,
};
pub static COLOR_DARK_GROUND: Color = Color {
    r: 0.19,
    g: 0.19,
    b: 0.59,
    a: 1.0,
};
pub static COLOR_LIGHT_GROUND: Color = Color {
    r: 0.78,
    g: 0.70,
    b: 0.19,
    a: 1.0,
};

pub static RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
pub static ORANGE: Color = Color { r: 1.0, g: 0.41, b: 0.0, a: 1.0 };
pub static LIGHT_GREEN: Color = Color { r: 0.33, g: 1.0, b: 0.19, a: 1.0 };
pub static LIGHT_YELLOW: Color = Color { r: 0.98, g: 1.0, b: 0.43, a: 1.0 };

pub static ROOM_MAX_SIZE: i32 = 10;
pub static ROOM_MIN_SIZE: i32 = 6;
pub static MAX_ROOMS: i32 = 30;

// pub static FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
pub static FOV_LIGHT_WALLS: bool = true;
pub static TORCH_RADIUS: i32 = 10;

pub static PLAYER: usize = 0;

pub static INVENTORY_WIDTH: i32 = 50;

pub static HEAL_AMOUNT: i32 = 40;

pub static LIGHTNING_DAMAGE: i32 = 40;
pub static LIGHTNING_RANGE: i32 = 5;

pub static CONFUSE_RANGE: i32 = 8;
pub static CONFUSE_NUM_TURNS: i32 = 10;

pub static FIREBALL_RADIUS: i32 = 3;
pub static FIREBALL_DAMAGE: i32 = 25;

pub static LEVEL_UP_BASE: i32 = 200;
pub static LEVEL_UP_FACTOR: i32 = 150;

pub static LEVEL_SCREEN_WIDTH: i32 = 40;
pub static CHARACTER_SCREEN_WIDTH: i32 = 30;
