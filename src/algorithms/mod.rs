use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

/// This trait is used to implement algorithms. The run function is used to continuously run the algorithm
/// The initial_step function is used to initialize the algorithm if needed. The next_step function is used
/// to improve upon any previously found solutions
pub trait Algorithm {
    fn run(&mut self, stop_signal: Arc<AtomicBool>) {
        self.initial_step();

        while !stop_signal.load(Ordering::Relaxed) {
            self.next_step();
            // Sleep for 100 millisecond to prevent the thread from using 100% of the CPU
            // This is useful for algorithms that finish extremely quickly
            thread::sleep(Duration::from_millis(100));
        }
    }

    fn initial_step(&mut self) {}

    fn next_step(&mut self);
}

pub mod hill_climber;
pub mod hill_climber_guided;
pub mod random_walker;
