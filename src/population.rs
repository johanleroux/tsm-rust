extern crate quickersort;
extern crate rand;

use config;
use simulation::models::{individual::Individual, location::Location};

#[derive(Clone)]
pub struct Population {
    individuals: Vec<Individual>,
    total_fitness: f64,
}

impl Population {
    pub fn new() -> Population {
        Population {
            total_fitness: 0.0,
            individuals: Vec::new(),
        }
    }

    pub fn init(&mut self, locations: &Vec<Location>) {
        self.individuals.clear();

        for _ in 0..config::POP_SIZE {
            let mut individual = Individual::new();
            individual.init(&locations);
            self.individuals.push(individual);
        }

        self.fitness();
    }

    pub fn fitness(&mut self) {
        self.total_fitness = 0.0;

        for i in 0..config::POP_SIZE {
            self.individuals[i].fitness();
            self.total_fitness += self.individuals[i].fitness;
        }
    }

    pub fn fittest(&mut self) -> Individual {
        let mut fittest = self.individuals[0];

        for i in 1..config::POP_SIZE {
            if self.individuals[i].fitness < fittest.fitness {
                fittest = self.individuals[i];
            }
        }
        fittest
    }

    pub fn avg_fitness(&mut self) -> f64 {
        self.total_fitness / config::POP_SIZE as f64
    }

    pub fn median_fitness(&mut self) -> f64 {
        let mut vals = Vec::new();
        for i in 0..config::POP_SIZE {
            vals.push(self.individuals[i].fitness);
        }
        quickersort::sort_floats(&mut vals[..]);

        vals[config::POP_SIZE / 2 as usize]
    }

    pub fn get_individual(&mut self, index: usize) -> Individual {
        self.individuals[index]
    }

    pub fn get_individuals(&mut self) -> Vec<Individual> {
        let mut tmp: Vec<Individual> = Vec::new();
        assert_eq!(tmp.len(), 0);

        for i in 0..config::POP_SIZE {
            tmp.push(self.individuals[i]);
        }

        assert_eq!(tmp.len(), config::POP_SIZE);
        tmp
    }

    pub fn set_individual(&mut self, index: usize, individual: Individual) {
        self.individuals[index] = individual;
    }

    pub fn set_individuals(&mut self, individuals: Vec<Individual>) {
        self.individuals.clear();
        for individual in individuals {
            self.individuals.push(individual);
        }
    }
}
