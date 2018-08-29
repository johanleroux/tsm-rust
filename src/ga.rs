use simulation::models::individual::Individual;
use population::Population;
use rand::prelude::*;
use config;
use utils;

pub struct GA;

impl GA {
    pub fn evolve(old_individuals: Vec<Individual>) -> Population {
        utils::debug("Initialize Variables");
        let mut pop: Population = Population::new();
        let mut individuals: Vec<Individual> = Vec::new();
        let mut rng = thread_rng();

        utils::debug("Loop through population for crossover");
        for _x in 0..config::POP_SIZE {
            let p1: usize = rng.gen_range(0, config::POP_SIZE - 1);
            let mut _p2: usize = p1;

            loop {
                _p2 = rng.gen_range(0, config::POP_SIZE - 1);

                if p1 != _p2 {
                    break;
                }            
            }

            utils::debug("Do crossover for {} individual");
            let child: Individual = GA::crossover(old_individuals[p1], old_individuals[_p2]);
            individuals.push(child);
        }
        utils::debug("Set new population");
        pop.set_individuals(individuals);

        utils::debug("Loop through population for mutation");
        for i in 0..config::POP_SIZE {
            let individual = pop.get_individual(i);
            pop.set_individual(i, GA::mutate(individual));
        }

        pop
    }

    fn crossover(p1: Individual, p2: Individual) -> Individual {
        let mut c: Individual = Individual::new();
        let mut rng = thread_rng();

        for i in 0..config::CITY_SIZE {
            c.set_location(i, p1.get_location(i))
        }

        for i in 0..config::CITY_SIZE {
            if rng.gen() {
                c.swap_location(i, p2.get_location(i))
            }
        }

        //println!("Doing crossover for individual");

        // let start_pos: usize = rng.gen_range(0, config::CITY_SIZE - 1);
        // let end_pos: usize = rng.gen_range(0, config::CITY_SIZE - 1);

        // for i in 0..config::CITY_SIZE {
        //     if start_pos < end_pos && i > start_pos && i < end_pos {
        //         c.set_location(i, p1.get_location(i))
        //     } else if start_pos > end_pos {
        //         if !(i < start_pos && i > end_pos) {
        //             c.set_location(i, p1.get_location(i))
        //         }
        //     }
        // }

        // for i in 0..config::CITY_SIZE {
        //     if !c.has_location(p2.get_location(i)) {
        //         for j in 0..config::CITY_SIZE {
        //             if c.get_location(j).x == -1.0 {
        //                 c.set_location(j, p2.get_location(i));
        //             }
        //         }
        //     }
        // }

        c.validate();

        c
    }

    fn mutate(individual: Individual) -> Individual {
        let mut ind = individual;
        let mut rng = thread_rng();

        if rng.gen::<f32>() < config::MUTATION_RATE {
            for c1 in 0..config::CITY_SIZE {
                let mut _c2 = c1;
                loop {
                    _c2 = rng.gen_range(0, config::CITY_SIZE - 1);

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