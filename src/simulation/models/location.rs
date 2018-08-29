use opengl_graphics::GlGraphics;
use piston_window::{rectangle, types, Context, Transformed};

use num;
use config::color;
use simulation::models::Drawable;

#[derive(Copy, Clone)]
pub struct Location {
    pub x: f64,
    pub y: f64,
}

impl Location {
    #[inline]
    pub fn new() -> Self {
        Location {
            x: -1.0, y: -1.0
        }
    }

    pub fn distance(&self, location: Location) -> f64 {
        let x_distance: f64 = num::abs(self.x - location.x);
        let y_distance: f64 = num::abs(self.y - location.y);

        num::Float::sqrt((x_distance * x_distance) + (y_distance * y_distance) as f64)
    }
}

const LOCATION_DIAMETER: f64 = 5.0;
impl Drawable for Location {
    fn draw(&self, context: Context, graphics: &mut GlGraphics) {
        const LOCATION: types::Rectangle = [0.0, 0.0, LOCATION_DIAMETER, LOCATION_DIAMETER];

        rectangle(
            color::WHITE,
            LOCATION,
            context.transform.trans(self.x, self.y),
            graphics,
        )
    }
}