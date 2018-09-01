use config;
use rand::prelude::*;
use simulation::models::individual::Individual;

pub struct Mutation {}

impl Mutation {
    pub fn mutate(individual: Individual) -> Individual {
        let mut ind = individual;
        let mut rng = thread_rng();

        for c1 in 0..config::LOCATION_SIZE {
            if rng.gen::<f32>() < config::MUTATION_RATE {
                let mut _c2 = c1;
                loop {
                    _c2 = rng.gen_range(0, config::LOCATION_SIZE - 1);

                    if c1 != _c2 {
                        break;
                    }            
                }

                let loc1 = ind.get_location(c1);
                let loc2 = ind.get_location(_c2);

                ind.set_location(c1, loc2);
                ind.set_location(_c2, loc1);
            }
        }

        ind.validate();

        ind
    }
}