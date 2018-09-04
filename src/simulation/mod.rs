use opengl_graphics::{GlGraphics, GlyphCache};
use piston_window::{
    clear, rectangle, text, Button, Context, EventLoop, EventSettings, Key, PistonWindow,
    PressEvent, RenderEvent, Transformed, UpdateEvent,
};
use rand::prelude::*;
pub mod models;
use self::models::{individual::Individual, location::Location, Drawable};
use config;
use config::color;
use ga;
use population::Population;
use utils;

#[derive(Clone)]
pub struct Simulation {
    locations: Vec<Location>,
    generation: u32,
    avg_fitness: f64,
    best_fitness: f64,
    best_individual: Individual,
    population: Population,
    stuck_generations: u32,
}

impl Simulation {
    pub fn new() -> Self {
        Simulation {
            locations: Vec::new(),
            generation: 1,
            avg_fitness: 0.0,
            best_fitness: 0.0,
            best_individual: Individual::new(),
            population: Population::new(),
            stuck_generations: 0,
        }
    }

    pub fn run(
        &mut self,
        window: &mut PistonWindow,
        opengl: &mut GlGraphics,
        glyph_cache: &mut GlyphCache,
    ) {
        let mut settings: EventSettings = EventSettings::new();
        unsafe {
            if config::BENCH_MODE {
                settings.set_max_fps(128);
                settings.set_ups(128);
                settings.set_ups_reset(0);
                settings.set_bench_mode(true);
            } else {
                settings.set_max_fps(64);
                settings.set_ups(64);
                settings.set_ups_reset(2);
                settings.set_bench_mode(false);
            }
        }
        window.set_event_settings(settings);

        // println!("Max FPS: {} ", window.get_event_settings().max_fps);
        // println!("UPS: {} ", window.get_event_settings().ups);
        // println!("UPS Reset: {} ", window.get_event_settings().ups_reset);
        // println!("Bench Mode: {} ", window.get_event_settings().bench_mode);

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

                    let line = format!(
                        "{},{},{},{}",
                        self.generation,
                        self.best_individual.fitness,
                        self.population.avg_fitness(),
                        self.population.median_fitness()
                    );
                    utils::write_to_file(line, true);
                } else {
                    utils::complete_file(self.best_fitness);
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

        for location in &self.locations {
            location.draw(context, graphics);
        }

        rectangle(
            [0.0, 0.0, 0.0, 0.5],
            [0.0, 0.0, 225.0, 215.0],
            context.transform.trans(0.0, 0.0),
            graphics,
        );

        text(
            color::WHITE,
            20,
            format!("Generation: {}", self.generation).as_str(),
            glyph_cache,
            context.transform.trans(10.0, 25.0),
            graphics,
        ).unwrap();

        text(
            color::WHITE,
            20,
            format!("Avg Fitness: {0:.02}", self.avg_fitness).as_str(),
            glyph_cache,
            context.transform.trans(10.0, 45.0),
            graphics,
        ).unwrap();

        text(
            color::WHITE,
            20,
            format!("Best Fitness: {0:.02}", self.best_fitness).as_str(),
            glyph_cache,
            context.transform.trans(10.0, 65.0),
            graphics,
        ).unwrap();

        let selection: String;
        unsafe {
            match config::SELECTION_ALGORITHM_X {
                config::SelectionAlgorithm::Tournament => {
                    selection = "Tournament".to_string();
                }
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
            context.transform.trans(10.0, 95.0),
            graphics,
        ).unwrap();
        unsafe {
            text(
                color::WHITE,
                20,
                format!("Elitism: {}%", config::ELITISM).as_str(),
                glyph_cache,
                context.transform.trans(10.0, 115.0),
                graphics,
            ).unwrap();
        }
        unsafe {
            text(
                color::WHITE,
                20,
                format!("Bench Mode: {}", config::BENCH_MODE).as_str(),
                glyph_cache,
                context.transform.trans(10.0, 135.0),
                graphics,
            ).unwrap();
        }

        text(
            color::WHITE,
            20,
            format!("Location Count: {}", config::LOCATION_SIZE).as_str(),
            glyph_cache,
            context.transform.trans(10.0, 165.0),
            graphics,
        ).unwrap();
        text(
            color::WHITE,
            20,
            format!("Population Size: {}", config::POP_SIZE).as_str(),
            glyph_cache,
            context.transform.trans(10.0, 185.0),
            graphics,
        ).unwrap();
        text(
            color::WHITE,
            20,
            format!("Mutation Rate: {}%", config::MUTATION_RATE * 100.0).as_str(),
            glyph_cache,
            context.transform.trans(10.0, 205.0),
            graphics,
        ).unwrap();
    }

    fn init(&mut self) {
        self.locations.clear();

        if config::TEST_DATA {
            self.locations = utils::test_data();
        } else {
            let mut rng = thread_rng();
            loop {
                let r_x: f64 = rng.gen_range(0, config::WINDOW_SIZE.width) as f64;
                let r_y: f64 = rng.gen_range(0, config::WINDOW_SIZE.height) as f64;

                if r_x < 225.0 && r_y < 215.0 {
                    continue;
                }

                self.locations.push(Location { x: r_x, y: r_y });

                // println!("x: {}, y: {}", r_x, r_y);

                if self.locations.len() == config::LOCATION_SIZE {
                    break;
                }
            }
        }

        utils::create_file();
        let mut line = format!("gen,high,avg,med");
        utils::write_to_file(line, false);

        self.generation = 1;
        self.population.init(&self.locations);
        self.best_individual = self.population.fittest();
        self.best_fitness = self.best_individual.fitness;
        self.avg_fitness = self.population.avg_fitness();

        line = format!(
            "{},{},{},{}",
            self.generation,
            self.best_individual.fitness,
            self.population.avg_fitness(),
            self.population.median_fitness()
        );
        utils::write_to_file(line, true);
    }
}
