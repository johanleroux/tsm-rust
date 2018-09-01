use opengl_graphics::GlGraphics;
use piston_window::Context;

pub mod individual;
pub mod location;

pub trait Drawable {
    fn draw(&self, context: Context, graphics: &mut GlGraphics);
}
