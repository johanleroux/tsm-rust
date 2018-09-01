use config;
use rand::prelude::*;
use simulation::models::individual::Individual;

pub struct Crossover {}

impl Crossover {
    pub fn order(p1: Individual, p2: Individual) -> Individual {
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

    pub fn partially_mapped(p1: Individual, p2: Individual) -> Individual {
        let mut c: Individual = Individual::new();
        let mut rng = thread_rng();

        let start_pos: usize = rng.gen_range(0, config::LOCATION_SIZE - 5);
        let end_pos: usize = rng.gen_range(start_pos, config::LOCATION_SIZE - 1);

        // Copy the mask from p1 directly to c
        for i in start_pos..end_pos {
            c.set_location(i, p1.get_location(i));
        }

        // Loop through mask and check what doesn't exist in the p2
        for i in start_pos..end_pos {
            if !c.has_location(p2.get_location(i)) {
                let _index = i; // p2.get_index_of_location(p1.get_location(i));
                let _value = p2.get_location(i);

                if _index < start_pos && _index > end_pos {
                    // c.set_location(index, location)
                }
            }
        }

        c.validate();

        c
    }
}
