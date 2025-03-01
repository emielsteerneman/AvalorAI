use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

pub trait Algorithm {
    fn run(&mut self, stop_signal: Arc<AtomicBool>) {
        self.initial_step();

        while !stop_signal.load(Ordering::Relaxed) {
            self.next_step();
            // Sleep for 1 millisecond to prevent the thread from using 100% of the CPU
            thread::sleep(Duration::from_millis(100));
        }
    }

    fn initial_step(&mut self);
    fn next_step(&mut self);
}

pub mod hill_climber;
pub mod random_walker;
