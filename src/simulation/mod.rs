use opengl_graphics::{GlGraphics, GlyphCache};
use piston_window::{clear, text, Button, Context, Key, PistonWindow, EventSettings, EventLoop, PressEvent, RenderEvent, Transformed, UpdateEvent};
use rand::prelude::*;
pub mod models;
use config::color;
use config;
use self::models::{location::Location, individual::Individual, Drawable};
use ga;
use utils;
use population::Population;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

#[derive(Clone)]
pub struct Simulation {
    cities: Vec<Location>,
    generation: u32,
    avg_fitness: f64,
    best_fitness: f64,
    cur_individual: Individual,
    best_individual: Individual,
    population: Population
}

impl Simulation {
    pub fn new() -> Self {
        Simulation {
            cities: Vec::new(),
            generation: 1,
            avg_fitness: 0.0,
            best_fitness: 0.0,
            cur_individual: Individual::new(),
            best_individual: Individual::new(),
            population: Population::new()
        }
    }

    pub fn run(&mut self, window: &mut PistonWindow, opengl: &mut GlGraphics, glyph_cache: &mut GlyphCache) {
        // window.set_event_settings(
        //     EventSettings::new().ups(128).ups_reset(0).bench_mode(true)
        // );

        let event_settings = window.get_event_settings();
        println!("UPS: {}", event_settings.ups);
        println!("UPS Reset: {}", event_settings.ups_reset);
        println!("Max FPS: {}", event_settings.max_fps);

        let path = Path::new("results.txt");
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}",
                            display,
                            why.description()),
            Ok(file) => file,
        };

        self.init();

        while let Some(event) = window.next() {
            if let Some(args) = event.render_args() {
                opengl.draw(args.viewport(), |context, graphics| {
                    self.draw(context, graphics, glyph_cache)
                });
            }

            if let Some(_args) = event.update_args() {
                utils::debug("Next Generation");

                if self.generation < 100 {
                    self.generation += 1;

                    utils::debug("Get individuals of current population");
                    let mut individuals = self.population.get_individuals();
                    utils::debug("Evolve population");
                    self.population = ga::GA::evolve(individuals);
                    self.population.fitness();
                    utils::debug("Calculate fittest");

                    self.cur_individual = self.population.fittest();
                    if self.cur_individual.fitness < self.best_individual.fitness
                    {
                        self.best_individual = self.population.fittest();
                        // self.best_individual.debug();
                        self.best_fitness = self.best_individual.fitness;
                        self.avg_fitness = self.population.avg_fitness();
                    }

                    println!("Best: {}", self.best_fitness);

                    let line = format!("{},{},{},{}\n", self.generation, self.cur_individual.fitness, self.population.avg_fitness(), self.population.median_fitness());

                    match file.write_all(line.as_bytes()) {
                        Err(why) => {
                            panic!("couldn't write to {}: {}", display, why.description())
                        },
                        Ok(_) => {},
                    }
                }
            }

            if let Some(Button::Keyboard(key)) = event.press_args() {
                match key {
                    Key::Escape => break,
                    _ => {}
                }
            }
        }
    }

    fn draw(&self, context: Context, graphics: &mut GlGraphics, glyph_cache: &mut GlyphCache) {
        clear(color::BLACK, graphics);
        for location in &self.cities {
            location.draw(context, graphics);
        }

        self.best_individual.draw(context, graphics);
        // self.cur_individual.draw_secondary(context, graphics);

        text(
            color::WHITE,
            20,
            format!("Generation: {}", self.generation).as_str(),
            glyph_cache,
            context.transform.trans(20.0, 40.0),
            graphics,
        ).unwrap();

        text(
            color::WHITE,
            20,
            format!("Avg Fitness: {0:.02}", self.avg_fitness).as_str(),
            glyph_cache,
            context.transform.trans(20.0, 60.0),
            graphics,
        ).unwrap();

        text(
            color::WHITE,
            20,
            format!("Best Fitness: {0:.02}", self.best_fitness).as_str(),
            glyph_cache,
            context.transform.trans(20.0, 80.0),
            graphics,
        ).unwrap();
    }

    fn init(&mut self) {
        self.cities.clear();
        // Test Data
        // self.cities.push(Location { x: 60.0, y: 200.0 });
        // self.cities.push(Location { x: 180.0, y: 200.0 });
        // self.cities.push(Location { x: 80.0, y: 180.0 });
        // self.cities.push(Location { x: 140.0, y: 180.0 });
        // self.cities.push(Location { x: 20.0, y: 160.0 });
        // self.cities.push(Location { x: 100.0, y: 160.0 });
        // self.cities.push(Location { x: 200.0, y: 160.0 });
        // self.cities.push(Location { x: 140.0, y: 140.0 });
        // self.cities.push(Location { x: 40.0, y: 120.0 });
        // self.cities.push(Location { x: 100.0, y: 120.0 });
        // self.cities.push(Location { x: 180.0, y: 100.0 });
        // self.cities.push(Location { x: 60.0, y: 80.0 });
        // self.cities.push(Location { x: 120.0, y: 80.0 });
        // self.cities.push(Location { x: 180.0, y: 60.0 });
        // self.cities.push(Location { x: 20.0, y: 40.0 });
        // self.cities.push(Location { x: 100.0, y: 40.0 });
        // self.cities.push(Location { x: 200.0, y: 40.0 });
        // self.cities.push(Location { x: 20.0, y: 20.0 });
        // self.cities.push(Location { x: 60.0, y: 20.0 });
        // self.cities.push(Location { x: 160.0, y: 20.0 });

        let mut rng = thread_rng();
        for _ in 0..config::LOCATION_SIZE {
            let r_x: f64 = rng.gen_range(0, config::WINDOW_SIZE.width) as f64;
            let r_y: f64 = rng.gen_range(0, config::WINDOW_SIZE.height) as f64;
            println!("{} - {}", r_x , r_y);

            self.cities.push(Location { x: r_x, y: r_y});
        }

        self.population.init(&self.cities);
        self.generation = 1;
        
        self.best_individual = self.population.fittest();
        self.best_fitness = self.best_individual.fitness;
        self.avg_fitness = self.population.avg_fitness();
    }
}
