use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

mod algorithms;
use algorithms::hill_climber::HillClimber;
use algorithms::hill_climber_guided::HillClimberGuided;
use algorithms::random_walker::RandomWalker;
use algorithms::Algorithm;

mod world;

mod data_structures;
use data_structures::problem::Problem;
use data_structures::solution::Solution;

mod visualisation;

fn main() {
    tracing_subscriber::fmt::init();
    color_eyre::install().expect("Failed to install color_eyre");

    // Create Problem and Solution
    let grid_size = 100;
    // let world = world::World::perlin(grid_size, 0.1);
    let world = world::World::random(grid_size, 0.0);

    let (start_x, start_y) = (
        rand::random_range(0..grid_size) as i64,
        rand::random_range(0..grid_size) as i64,
    );
    let problem: Problem = Problem::new(world.clone(), 15000, 1000, start_y, start_x);
    let solution = Arc::new(Solution::default());

    tracing::info!(
        "Problem created. N={} world, T={}ms, n={} steps, y={}, x={}",
        grid_size,
        problem.milliseconds,
        problem.n_steps,
        start_y,
        start_x,
    );

    // Create visualizer
    let visualizer = visualisation::window::MyWindow::new(problem.clone(), solution.clone());
    let visualizer_thread = visualizer.run(60);

    // Create hill climber and random walker
    let mut hc = HillClimber::new(problem.clone(), solution.clone());
    let mut rw = RandomWalker::new(problem.clone(), solution.clone());
    let mut hcb = HillClimberGuided::new(problem.clone(), solution.clone());

    // Shared stop signal between threads
    let stop_signal = Arc::new(AtomicBool::new(false));

    // Start threads for all three algorithms
    let stop_signal_copy = Arc::clone(&stop_signal);
    let hc_thread = thread::spawn(move || {
        hc.run(stop_signal_copy);
    });
    let stop_signal_copy = Arc::clone(&stop_signal);
    let rw_thread = thread::spawn(move || {
        rw.run(stop_signal_copy);
    });
    let stop_signal_copy = Arc::clone(&stop_signal);
    let hcg_thread = thread::spawn(move || {
        hcb.run(stop_signal_copy);
    });

    // Main thread: Timeout after T milliseconds
    thread::sleep(Duration::from_millis(problem.milliseconds));
    tracing::info!("Timeout reached! Stopping worker thread.");

    // Signal the worker thread to stop
    stop_signal.store(true, Ordering::Relaxed);

    // Get final path from solution
    let _: Vec<(i64, i64)> = solution.path.lock().unwrap().clone();
    tracing::info!("Final path collected by main thread with score {}", solution.get_score());

    // Wait for the worker to finish
    hc_thread.join().expect("HC thread failed to join.");
    rw_thread.join().expect("RW thread failed to join.");
    hcg_thread.join().expect("HCG thread failed to join.");
    visualizer_thread
        .join()
        .expect("Visualizer thread failed to join.");
}
