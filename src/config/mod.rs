use piston_window::Size;

pub mod color;
pub mod font;

pub const WINDOW_SIZE: Size = Size {
    width: 1280,
    height: 720,
};

pub const DEBUG: bool = false;

pub const TEST_DATA: bool = true;
pub const TEST_RUNS: u32 = 10;

pub const MUTATION_RATE: f32 = 0.015;

pub const LOCATION_SIZE: usize = 100;
pub const POP_SIZE: usize = 300;

pub enum SelectionAlgorithm {
    Tournament,
    Roulette,
    Random,
}
pub static mut SELECTION_ALGORITHM_X: SelectionAlgorithm = SelectionAlgorithm::Tournament;

pub static mut ELITISM: usize = 1;

pub static mut BENCH_MODE: bool = true;
