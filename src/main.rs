#![allow(non_snake_case)]

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

mod algorithms;
use algorithms::hill_climber::HillClimber;
use algorithms::random_walker::RandomWalker;
use algorithms::Algorithm;

mod world;

mod data_structures;
use data_structures::problem::Problem;
use data_structures::solution::Solution;

fn main() {
    tracing_subscriber::fmt::init();
    color_eyre::install().expect("Failed to install color_eyre");

    // Shared stop signal between threads
    let stop_signal = Arc::new(AtomicBool::new(false));

    // Create Problem and Solution
    let N = 100;
    let world = world::World::random(N, 0.);
    let (start_x, start_y) = (
        rand::random_range(0..N) as i64,
        rand::random_range(0..N) as i64,
    );
    let problem: Problem = Problem::new(world.clone(), 1000, 20, start_y, start_x);
    let solution = Arc::new(Solution::default());

    tracing::info!(
        "Problem created. N={} world, T={}ms, n={} steps, y={}, x={}",
        N,
        problem.milliseconds,
        problem.n_steps,
        start_y,
        start_x
    );

    // Create hill climber and random walker
    let mut hc = HillClimber::new(problem.clone(), solution.clone());
    let mut rw = RandomWalker::new(problem.clone(), solution.clone());

    // Start threads for hill climber and random walker
    let stop_signal_copy = Arc::clone(&stop_signal);
    let hc_thread = thread::spawn(move || {
        hc.run(stop_signal_copy);
    });
    let stop_signal_copy = Arc::clone(&stop_signal);
    let rw_thread = thread::spawn(move || {
        rw.run(stop_signal_copy);
    });

    // Main thread: Timeout after T milliseconds
    thread::sleep(Duration::from_millis(problem.milliseconds));
    tracing::info!("Timeout reached! Stopping worker thread.");

    // Signal the worker thread to stop
    stop_signal.store(true, Ordering::Relaxed);

    // Get final path from solution
    let final_path: Vec<(i64, i64)> = solution.path.lock().unwrap().clone();
    tracing::info!("Final vector collected by main thread: {:?}", final_path);

    // Wait for the worker to finish
    hc_thread.join().expect("HC thread failed to join.");
    rw_thread.join().expect("RW thread failed to join.");
}
