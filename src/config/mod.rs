use piston_window::Size;

pub mod color;
pub mod font;

pub const WINDOW_SIZE : Size = Size {
    width: 1280,
    height: 720,
};


pub const CITY_SIZE: usize = 20;
pub const POP_SIZE: usize = 50;
pub const MUTATION_RATE: f32 = 0.015;