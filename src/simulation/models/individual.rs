use opengl_graphics::GlGraphics;
use piston_window::{line, types, Context, Transformed};
use std::f64;
use rand::prelude::*;

use config;
use config::color;
use simulation::models::{location::Location, Drawable};

const TRAVEL_RADIUS: f64 = 1.0;

#[derive(Copy, Clone)]
pub struct Individual {
    pub fitness: f64,
    locations: [Location; config::LOCATION_SIZE],
    draw_secondary: bool
}

impl Individual {
    pub fn new() -> Individual {
        Individual {
            fitness: f64::MAX,
            locations: [Location::new(); config::LOCATION_SIZE],
            draw_secondary: false
        }
    }

    pub fn init(&mut self, cities: &Vec<Location>) {
        assert_eq!(cities.len(), config::LOCATION_SIZE);
        let mut rng = thread_rng();

        self.set_locations(cities);

        for _ in 0..100 {
            for c in 0..config::LOCATION_SIZE {
                let r: usize = rng.gen_range(0, config::LOCATION_SIZE);
                if r != c
                {
                    let tmp_location = self.locations[c];
                    self.locations[c] = self.locations[r];
                    self.locations[r] = tmp_location;
                }
            }
        }
    }

    pub fn fitness(&mut self) -> f64{
        let mut distance: f64 = 0.0;

        for i in 0..config::LOCATION_SIZE {
            let from = self.locations[i];
            let to = 
                if i + 1 < config::LOCATION_SIZE {
                    self.locations[i + 1]
                } else {
                    self.locations[0]
                };
            
            distance += from.distance(to);
        }

        self.fitness = distance;

        distance
    }

    pub fn get_location(&self, index: usize) -> Location {
        return self.locations[index]
    }

    pub fn set_location(&mut self, index: usize, location: Location) {
        self.locations[index] = location
    }

    pub fn set_locations(&mut self, locations: &Vec<Location>) {
        for i in 0..locations.len() {
            self.locations[i] = locations[i];
        }
    }

    pub fn has_location(&self, location: Location) -> bool {
        for i in 0..config::LOCATION_SIZE {
            if self.get_location(i).x == location.x && self.get_location(i).y == location.y {
                return true;
            }
        }
        false
    }

    pub fn swap_location(&mut self, index: usize, location: Location) {
        for i in 0..config::LOCATION_SIZE {
            if self.get_location(i).x == location.x && self.get_location(i).y == location.y {
                self.locations[i] = self.locations[index];
            }
        }

        self.locations[index] = location;
    }

    pub fn validate(&self) {
        for i in 0..config::LOCATION_SIZE {
            for u in 0..config::LOCATION_SIZE {
                if 
                    (i != u) &&
                    (self.locations[i].x == self.locations[u].x) &&
                    (self.locations[i].y == self.locations[u].y) {
                    println!("Individual has the same locations x-{} y-{} | x-{} y-{}", self.locations[i].x, self.locations[i].y, self.locations[u].x, self.locations[u].y);
                    self.debug();
                    assert_eq!(0, 1)
                }
            }
        }
    }

    pub fn draw_x(self, x: f64, secondary: bool) -> f64 {
        if secondary {
            return (x) + (config::WINDOW_SIZE.width / 2) as f64
        }
        x
    }

    pub fn draw_y(self, y: f64) -> f64 {
        y
    }

    pub fn debug(&self) {
        for i in 0..config::LOCATION_SIZE {
            println!(
                "Location: x-{} | y-{}", 
                self.locations[i].x,
                self.locations[i].y
            );
        }
    }

    pub fn draw_secondary(&self, context: Context, graphics: &mut GlGraphics) {
        let mut from_location = self.locations[0];
        for i in 1..config::LOCATION_SIZE {
            let line_angle: types::Line = [
                0.0,
                0.0,
                self.draw_x(from_location.x, true) - self.draw_x(self.locations[i].x, true), 
                self.draw_y(from_location.y) - self.draw_y(self.locations[i].y), 
            ];
            line(
                color::CYAN,
                TRAVEL_RADIUS,
                line_angle,
                context.transform.trans(self.draw_x(self.locations[i].x, true), self.draw_y(self.locations[i].y)),
                graphics
            );

            from_location = self.locations[i];
        }

        let line_angle: types::Line = [
            0.0,
            0.0,
            self.draw_x(from_location.x, true) - self.draw_x(self.locations[0].x, true),
            self.draw_y(from_location.y) - self.draw_y(self.locations[0].y)
        ];
        line(
            color::CYAN,
            TRAVEL_RADIUS,
            line_angle,
            context.transform.trans(self.draw_x(self.locations[0].x, true), self.draw_y(self.locations[0].y)),
            graphics
        );
    }
}

impl Drawable for Individual {
    fn draw(&self, context: Context, graphics: &mut GlGraphics) {
        let mut from_location = self.locations[0];
        for i in 1..config::LOCATION_SIZE {
            let line_angle: types::Line = [
                0.0,
                0.0,
                self.draw_x(from_location.x, false) - self.draw_x(self.locations[i].x, false), 
                self.draw_y(from_location.y) - self.draw_y(self.locations[i].y), 
            ];
            line(
                color::CYAN,
                TRAVEL_RADIUS,
                line_angle,
                context.transform.trans(self.draw_x(self.locations[i].x, false), self.draw_y(self.locations[i].y)),
                graphics
            );

            from_location = self.locations[i];
        }

        let line_angle: types::Line = [
            0.0,
            0.0,
            self.draw_x(from_location.x, false) - self.draw_x(self.locations[0].x, false),
            self.draw_y(from_location.y) - self.draw_y(self.locations[0].y)
        ];
        line(
            color::CYAN,
            TRAVEL_RADIUS,
            line_angle,
            context.transform.trans(self.draw_x(self.locations[0].x, false), self.draw_y(self.locations[0].y)),
            graphics
        );
    }
}