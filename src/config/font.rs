use piston_window::types;
use config::color;

pub const TITLE_PADDING: f64 = 112.0;
pub const TITLE_SIZE: u32 = 44;

pub const PADDING: f64 = 64.0;
pub const SIZE: u32 = 32;

pub fn color(menu: i32, index: usize) -> types::Color  {
    if menu == index as i32 {
        return color::CYAN;
    }
    return color::WHITE;
}