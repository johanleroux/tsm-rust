use opengl_graphics::{GlGraphics, GlyphCache};
use piston_window::{clear, text, Button, Context, Key, PistonWindow, PressEvent, RenderEvent, Transformed, UpdateEvent};
use rand::prelude::*;
pub mod models;
use config::color;
use config;
use self::models::{location::Location, individual::Individual, Drawable};
use ga;
use utils;
use population::Population;

#[derive(Clone)]
pub struct Simulation {
    cities: Vec<Location>,
    generation: u32,
    avg_fitness: f64,
    best_fitness: f64,
    best_individual: Individual,
    population: Population,
    stuck_generations: u32
}

impl Simulation {
    pub fn new() -> Self {
        Simulation {
            cities: Vec::new(),
            generation: 1,
            avg_fitness: 0.0,
            best_fitness: 0.0,
            best_individual: Individual::new(),
            population: Population::new(),
            stuck_generations: 0
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

            if let Some(_args) = event.update_args() {
                if self.stuck_generations < 25 {
                    self.generation += 1;

                    let mut individuals = self.population.get_individuals();
                    self.population = ga::GA::evolve(individuals);

                    self.population.fitness();

                    self.best_individual = self.population.fittest();

                    if self.best_fitness == self.best_individual.fitness {
                        self.stuck_generations += 1
                    } else {
                        self.stuck_generations = 0
                    }

                    self.best_fitness = self.best_individual.fitness;
                    self.avg_fitness = self.population.avg_fitness();

                    let line = format!("{},{},{},{}", self.generation, self.best_individual.fitness, self.population.avg_fitness(), self.population.median_fitness());
                    utils::write_to_file(line, true);
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

        self.best_individual.draw(context, graphics);

        for location in &self.cities {
            location.draw(context, graphics);
        }

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

        let selection: String;
        unsafe {
            match config::SELECTION_ALGORITHM_X {
                config::SelectionAlgorithm::Tournament => {
                    selection = "Tournament".to_string();
                },
                config::SelectionAlgorithm::Roulette => {
                    selection = "Roulette".to_string();
                }
                config::SelectionAlgorithm::Random => {
                    selection = "Random".to_string();
                }
            }
        } 

        text(
            color::WHITE,
            20,
            format!("Selection: {}", selection).as_str(),
            glyph_cache,
            context.transform.trans(20.0, 100.0),
            graphics,
        ).unwrap();
    }

    fn init(&mut self) {
        self.cities.clear();

        let mut rng = thread_rng();
        for _ in 0..config::LOCATION_SIZE {
            let r_x: f64 = rng.gen_range(0, config::WINDOW_SIZE.width) as f64;
            let r_y: f64 = rng.gen_range(0, config::WINDOW_SIZE.height) as f64;
            self.cities.push(Location { x: r_x, y: r_y});
        }

        utils::create_file();
        let mut line = format!("gen,high,avg,med");
        utils::write_to_file(line, false);

        self.generation = 1;
        self.population.init(&self.cities);
        self.best_individual = self.population.fittest();
        self.best_fitness = self.best_individual.fitness;
        self.avg_fitness = self.population.avg_fitness();

        line = format!("{},{},{},{}", self.generation, self.best_individual.fitness, self.population.avg_fitness(), self.population.median_fitness());
        utils::write_to_file(line, true);
    }
}
