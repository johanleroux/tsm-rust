use simulation::models::individual::Individual;
use population::Population;
use config;
use utils;
mod select;
use self::select::Select;
mod crossover;
use self::crossover::Crossover;
mod mutation;
use self::mutation::Mutation;

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
        let mut min_fitness = old_individuals[0].fitness;
        unsafe {
            match config::SELECTION_ALGORITHM_X {
                config::SelectionAlgorithm::Roulette => {
                    for i in 1..config::POP_SIZE {
                        if min_fitness > old_individuals[i].fitness {
                            min_fitness = old_individuals[i].fitness
                        }
                    }
                },
                _ => {}
            }
        }

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
                        p1 = Select::roulette(&old_individuals, min_fitness);
                        p2 = Select::roulette(&old_individuals, min_fitness);
                    },
                    config::SelectionAlgorithm::Random => {
                        let tmp: Individual = Individual::new();
                        p1 = Select::random(&old_individuals, tmp);
                        p2 = Select::random(&old_individuals, p1);
                    }
                }
            }

            utils::debug("Do crossover for {} individual");
            let child: Individual = Crossover::order(p1, p2);
            if false {
                Crossover::partially_mapped(p1, p2);
            }
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
            pop.set_individual(i, Mutation::mutate(individual));
        }

        pop
    }
}