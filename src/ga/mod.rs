use simulation::models::individual::Individual;
use population::Population;
use rand::prelude::*;
use config;
use utils;
mod select;
use self::select::Select;

pub struct GA;

impl GA {
    pub fn evolve(old_individuals: Vec<Individual>) -> Population {
        utils::debug("Initialize Variables");
        let mut pop: Population = Population::new();
        let mut individuals: Vec<Individual> = Vec::new();

        unsafe {
            if config::ELITISM > 0 {
                individuals.clear();
                individuals = Select::elitism(&old_individuals, (config::POP_SIZE as f64 * (config::ELITISM as f64 / 100.0)) as usize);
            }
        }

        utils::debug("Loop through population for crossover");
        loop {
            let p1: Individual;
            let p2: Individual;

            unsafe {
                match config::SELECTION_ALGORITHM_X {
                    config::SelectionAlgorithm::Tournament => {
                        p1 = Select::tour(&old_individuals);
                        p2 = Select::tour(&old_individuals);
                    },
                    config::SelectionAlgorithm::Roulette => {
                        p1 = Select::roulette(&old_individuals);
                        p2 = Select::roulette(&old_individuals);
                    },
                    config::SelectionAlgorithm::Random => {
                        let tmp: Individual = Individual::new();
                        p1 = Select::random(&old_individuals, tmp);
                        p2 = Select::random(&old_individuals, p1);
                    }
                }
            }

            utils::debug("Do crossover for {} individual");
            let child: Individual = GA::crossover(p1, p2);
            individuals.push(child);

            if individuals.len() == config::POP_SIZE {
                break;
            }
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

        let start_pos: usize = rng.gen_range(0, config::LOCATION_SIZE - 1);
        let end_pos: usize = rng.gen_range(0, config::LOCATION_SIZE - 1);

        for i in 0..config::LOCATION_SIZE {
            if (start_pos < end_pos) && (i > start_pos) && (i < end_pos) {
                c.set_location(i, p1.get_location(i))
            } else if start_pos > end_pos {
                if !((i < start_pos) && (i > end_pos)) {
                    c.set_location(i, p1.get_location(i))
                }
            }
        }

        for i in 0..config::LOCATION_SIZE {
            if !c.has_location(p2.get_location(i)) {
                for j in 0..config::LOCATION_SIZE {
                    if c.get_location(j).x == -1.0 {
                        c.set_location(j, p2.get_location(i));
                        break;
                    }
                }
            }
        }

        c.validate();

        c
    }

    fn mutate(individual: Individual) -> Individual {
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