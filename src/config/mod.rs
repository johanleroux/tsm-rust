use piston_window::Size;

pub mod color;
pub mod font;

pub const WINDOW_SIZE : Size = Size {
    width: 1280,
    height: 720,
};


pub const LOCATION_SIZE: usize = 30;
pub const POP_SIZE: usize = 200;
pub const MUTATION_RATE: f32 = 0.015;