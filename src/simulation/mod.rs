use opengl_graphics::{GlGraphics, GlyphCache};
use piston_window::{clear, types, ellipse, line, text, Button, Context, Key, PistonWindow, PressEvent, ReleaseEvent, RenderEvent, Transformed};

use config::color;
use self::models::{city, travel, Drawable};
mod models;

pub struct Simulation {
    cities: Vec<city::City>,
    travel: travel::Travel
}

impl Simulation {
    pub fn new() -> Self {
        Simulation {
            cities: Vec::new(),
            travel: travel::Travel::default(),
        }
    }

    pub fn run(&mut self, window: &mut PistonWindow, opengl: &mut GlGraphics, glyph_cache: &mut GlyphCache) {
        self.init();

        while let Some(event) = window.next() {
            if let Some(args) = event.render_args() {
                opengl.draw(args.viewport(), |context, graphics| {
                    self.draw(context, graphics, glyph_cache)
                });
            }

            if let Some(Button::Keyboard(key)) = event.press_args() {
                match key {
                    Key::Escape => break,
                    _ => {}
                }
            }

            if let Some(Button::Keyboard(key)) = event.release_args() {
                
            }
        }
    }

    fn draw(&self, context: Context, graphics: &mut GlGraphics, glyph_cache: &mut GlyphCache) {
        clear(color::BLACK, graphics);
        for city in &self.cities {
            city.draw(context, graphics);
        }

        self.travel.draw(context, graphics);

        text(
            color::WHITE,
            20,
            format!("Generation: {}", 0).as_str(),
            glyph_cache,
            context.transform.trans(20.0, 40.0),
            graphics,
        ).unwrap();

        text(
            color::WHITE,
            20,
            format!("Avg Distance: {}", 0).as_str(),
            glyph_cache,
            context.transform.trans(20.0, 60.0),
            graphics,
        ).unwrap();

        text(
            color::WHITE,
            20,
            format!("Best Distance: {}", 0).as_str(),
            glyph_cache,
            context.transform.trans(20.0, 80.0),
            graphics,
        ).unwrap();
    }

    fn init(&mut self) {
        self.cities.push(city::City { x: 60.0, y: 200.0 });
        self.cities.push(city::City { x: 180.0, y: 200.0 });
        self.cities.push(city::City { x: 80.0, y: 180.0 });
        self.cities.push(city::City { x: 140.0, y: 180.0 });
        self.cities.push(city::City { x: 20.0, y: 160.0 });
        self.cities.push(city::City { x: 100.0, y: 160.0 });
        self.cities.push(city::City { x: 200.0, y: 160.0 });
        self.cities.push(city::City { x: 140.0, y: 140.0 });
        self.cities.push(city::City { x: 40.0, y: 120.0 });
        self.cities.push(city::City { x: 100.0, y: 120.0 });
        self.cities.push(city::City { x: 180.0, y: 100.0 });
        self.cities.push(city::City { x: 60.0, y: 80.0 });
        self.cities.push(city::City { x: 120.0, y: 80.0 });
        self.cities.push(city::City { x: 180.0, y: 60.0 });
        self.cities.push(city::City { x: 20.0, y: 40.0 });
        self.cities.push(city::City { x: 100.0, y: 40.0 });
        self.cities.push(city::City { x: 200.0, y: 40.0 });
        self.cities.push(city::City { x: 20.0, y: 20.0 });
        self.cities.push(city::City { x: 60.0, y: 20.0 });
        self.cities.push(city::City { x: 160.0, y: 20.0 });

        self.travel.set_cities(&self.cities);
    }
}
