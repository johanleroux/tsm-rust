use opengl_graphics::GlGraphics;
use piston_window::{line, types, Context, Transformed};

use config::color;
use simulation::models::{city, Drawable};

#[derive(Copy, Clone)]
pub struct Travel {
    travel: [city::City; 20],
}

impl Travel {
    #[inline]
    pub fn default() -> Self {
        Travel {
            travel: [city::City::default(); 20]
        }
    }

    pub fn set_cities(&mut self, cities: &Vec<city::City>) {
        for i in 0..cities.len()
        {
            self.travel[i] = cities[i];
        }
    }
}

const TRAVEL_RADIUS: f64 = 1.0;
impl Drawable for Travel {
    fn draw(&self, context: Context, graphics: &mut GlGraphics) {
        let mut prev_city = self.travel[0];
        for i in 1..self.travel.len() {
            let line_angle: types::Line = [
                0.0,
                0.0,
                prev_city.x - self.travel[i].x, 
                prev_city.y - self.travel[i].y
            ];
            line(
                color::CYAN,
                TRAVEL_RADIUS,
                line_angle,
                context.transform.trans(self.travel[i].x + 3.0, self.travel[i].y + 3.0),
                graphics
            );

            prev_city = self.travel[i];
        }
    }
}