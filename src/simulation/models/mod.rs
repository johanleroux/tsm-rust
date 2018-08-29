use opengl_graphics::GlGraphics;
use piston_window::{Context};

pub mod city;
pub mod travel;

pub trait Drawable {
    fn draw(&self, context: Context, graphics: &mut GlGraphics);
}
