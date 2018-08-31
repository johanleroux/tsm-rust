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
            
            if tmp.fitness() < candidate.fitness() {
                candidate = tmp;
            }
        }

        candidate
    }

    pub fn roulette(individuals: &Vec<Individual>) -> Individual {
        let mut rng = thread_rng();
        
        individuals[rng.gen_range(0, config::POP_SIZE - 1)]
    }
}