use opengl_graphics::{GlGraphics, GlyphCache};
use piston_window::{clear, text, Button, Context, Key, PistonWindow, PressEvent, RenderEvent, Transformed, UpdateEvent};
pub mod models;
use config::color;
use self::models::{city::City, individual::Individual, Drawable};
use ga;
use utils;
use population::Population;

#[derive(Clone)]
pub struct Simulation {
    cities: Vec<City>,
    generation: u32,
    avg_fitness: f64,
    best_fitness: f64,
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
            best_individual: Individual::new(),
            population: Population::new()
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
                utils::debug("Next Generation");

                self.generation += 1;
                
                {
                    utils::debug("Get individuals of current population");
                    let mut individuals = self.population.get_individuals();
                    utils::debug("Evolve population");
                    self.population = ga::GA::evolve(individuals);
                    self.population.fitness();
                    utils::debug("Calculate fittest");

                    let gen_best = self.population.fittest();
                    if gen_best.fitness < self.best_individual.fitness
                    {
                        self.best_individual = self.population.fittest();
                        // self.best_individual.debug();
                        self.best_fitness = self.best_individual.fitness;
                        self.avg_fitness = self.population.avg_fitness();
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
        for city in &self.cities {
            city.draw(context, graphics);
        }

        self.best_individual.draw(context, graphics);

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
        self.cities.push(City { x: 60.0, y: 200.0 });
        self.cities.push(City { x: 180.0, y: 200.0 });
        self.cities.push(City { x: 80.0, y: 180.0 });
        self.cities.push(City { x: 140.0, y: 180.0 });
        self.cities.push(City { x: 20.0, y: 160.0 });
        self.cities.push(City { x: 100.0, y: 160.0 });
        self.cities.push(City { x: 200.0, y: 160.0 });
        self.cities.push(City { x: 140.0, y: 140.0 });
        self.cities.push(City { x: 40.0, y: 120.0 });
        self.cities.push(City { x: 100.0, y: 120.0 });
        self.cities.push(City { x: 180.0, y: 100.0 });
        self.cities.push(City { x: 60.0, y: 80.0 });
        self.cities.push(City { x: 120.0, y: 80.0 });
        self.cities.push(City { x: 180.0, y: 60.0 });
        self.cities.push(City { x: 20.0, y: 40.0 });
        self.cities.push(City { x: 100.0, y: 40.0 });
        self.cities.push(City { x: 200.0, y: 40.0 });
        self.cities.push(City { x: 20.0, y: 20.0 });
        self.cities.push(City { x: 60.0, y: 20.0 });
        self.cities.push(City { x: 160.0, y: 20.0 });

        self.population.init(&self.cities);
        self.generation = 1;
        
        self.best_individual = self.population.fittest();
        self.best_fitness = self.best_individual.fitness;
        self.avg_fitness = self.population.avg_fitness();
    }
}
