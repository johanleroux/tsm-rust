use opengl_graphics::GlGraphics;
use piston_window::{Context};

pub mod location;
pub mod individual;

pub trait Drawable {
    fn draw(&self, context: Context, graphics: &mut GlGraphics);
}
