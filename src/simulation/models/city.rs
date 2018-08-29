use opengl_graphics::GlGraphics;
use piston_window::{rectangle, types, Context, Transformed};

use config::color;
use simulation::models::Drawable;

#[derive(Copy, Clone)]
pub struct City {
    pub x: f64,
    pub y: f64,
}

impl City {
    #[inline]
    pub fn default() -> Self {
        City {
            x: -1.0, y: -1.0
        }
    }
}

const CITY_DIAMETER: f64 = 5.0;
impl Drawable for City {
    fn draw(&self, context: Context, graphics: &mut GlGraphics) {
        const CITY: types::Rectangle = [0.0, 0.0, CITY_DIAMETER, CITY_DIAMETER];

        rectangle(
            color::WHITE,
            CITY,
            context.transform.trans(self.x, self.y),
            graphics,
        )
    }
}