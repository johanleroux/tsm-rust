use config;
use rand::prelude::*;
use simulation::models::individual::Individual;

pub struct Select {}

impl Select {
    pub fn tour(individuals: &Vec<Individual>) -> Individual {
        let mut rng = thread_rng();
        let mut candidate: Individual = individuals[rng.gen_range(0, config::POP_SIZE - 1)];

        for _ in 1..config::POP_SIZE {
            let mut tmp = individuals[rng.gen_range(0, config::POP_SIZE - 1)];
            loop {
                if !tmp.same_individual(candidate) {
                    break;
                }
                tmp = individuals[rng.gen_range(0, config::POP_SIZE - 1)];
            }
            
            if tmp.fitness() < candidate.fitness() {
                candidate = tmp;
            }
        }

        candidate
    }

    pub fn roulette(individuals: &Vec<Individual>, min_fitness: f64) -> Individual {
        let mut rng = thread_rng();
        
        let mut sum_of_probability: f64 = 0.0;
        for i in 1..config::POP_SIZE {
            sum_of_probability += Select::probability(min_fitness, individuals[i].fitness);
        }

        let mut r = rng.gen::<f64>() * sum_of_probability;

        for i in 1..config::POP_SIZE {
            r -= Select::probability(min_fitness, individuals[i].fitness);
            
            if r < 0.0 {
                return individuals[i]
            }
        }
        
        return individuals[config::POP_SIZE - 1]
    }

    fn probability(min_fitness: f64, cur_fitness: f64) -> f64 {
        return 1.0 / (1.0 + cur_fitness - min_fitness);
    }

    pub fn random(individuals: &Vec<Individual>, other: Individual) -> Individual {
        let mut rng = thread_rng();
        let mut candidate: Individual;
        
        loop {
            candidate = individuals[rng.gen_range(0, config::POP_SIZE - 1)];

            if !candidate.same_individual(other) {
                break;
            }
        }

        candidate
    }

    pub fn elitism(old_individuals: &Vec<Individual>, limit: usize) -> Vec<Individual> {
        let mut tmp_individuals: Vec<Individual> = Vec::new();
        let mut candidates: Vec<Individual> = Vec::new();

        for i in 0..old_individuals.len() {
            tmp_individuals.push(old_individuals[i])
        }

        tmp_individuals.sort_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap().reverse());

        for i in 0..limit {
            candidates.push(tmp_individuals[i])
        }

        candidates
    }
}