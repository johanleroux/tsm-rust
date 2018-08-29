use config;
use rand::prelude::*;
use simulation::models::individual::Individual;

pub struct select;

impl select {
    pub fn tour(individuals: &Vec<Individual>) -> Individual {
        let mut rng = thread_rng();
        let mut candidate: Individual = individuals[rng.gen_range(0, config::POP_SIZE - 1)];

        for i in 1..config::POP_SIZE {
            let mut tmp = individuals[rng.gen_range(0, config::POP_SIZE - 1)];
            
            if tmp.fitness() < candidate.fitness() {
                candidate = tmp;
            }
        }

        candidate
    }

    pub fn roulette(individuals: Vec<Individual>) -> Vec<Individual> {
        let mut candidates: Vec<Individual> = Vec::new();
        let mut rng = thread_rng();
        assert_eq!(candidates.len(), 0);

        for i in 0..config::POP_SIZE {
            candidates.push(individuals[rng.gen_range(0, config::POP_SIZE - 1)]);
        }

        assert_eq!(candidates.len(), config::POP_SIZE);
        candidates
    }
}