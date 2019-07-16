use tcod::colors::Color;
use tcod::map::FovAlgorithm;

pub static LIMIT_FPS: i32 = 60;

pub static SCREEN_WIDTH: i32 = 80;
pub static SCREEN_HEIGHT: i32 = 50;

pub static BAR_WIDTH: i32 = 20;
pub static PANEL_HEIGHT: i32 = 7;
pub static PANEL_Y: i32 = SCREEN_HEIGHT - PANEL_HEIGHT;

pub static MSG_X:     i32 = BAR_WIDTH + 2;
pub static MSG_WIDTH: i32 = SCREEN_WIDTH - BAR_WIDTH -2;
pub static MSG_HEIGHT: usize = PANEL_HEIGHT as usize -1;

pub static MAP_WIDTH: i32 = 80;
pub static MAP_HEIGHT: i32 = 45;

pub static COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
pub static COLOR_LIGHT_WALL: Color = Color { r: 130, g: 110, b:50 };
pub static COLOR_DARK_GROUND: Color = Color { r: 50, g: 50, b: 150 };
pub static COLOR_LIGHT_GROUND: Color = Color { r: 200, g: 180, b: 50 };

pub static ROOM_MAX_SIZE: i32 = 10;
pub static ROOM_MIN_SIZE: i32 = 6;
pub static MAX_ROOMS: i32 = 30;

pub static MAX_ROOM_MONSTERS: i32 = 3;
pub static MAX_ROOM_ITEMS: i32 = 2;

pub static FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
pub static FOV_LIGHT_WALLS: bool = true;
pub static TORCH_RADIUS: i32 = 10;

pub static PLAYER: usize = 0;

pub static INVENTORY_WIDTH: i32 = 50;
pub static HEAL_AMOUNT: i32 = 4;

pub static LIGHTNING_DAMAGE: i32 = 20;
pub static LIGHTNING_RANGE: i32 = 5;

pub static CONFUSE_RANGE: i32 = 8;
pub static CONFUSE_NUM_TURNS: i32 = 10;

pub static FIREBALL_RADIUS: i32 = 3;
pub static FIREBALL_DAMAGE: i32 = 12;
