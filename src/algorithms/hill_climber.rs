use crate::data_structures::{problem::Problem, solution::Solution};
use derive_more::Constructor;
use rand::Rng;
use std::{sync::Arc, thread, time::Duration};

use super::Algorithm;

#[derive(Constructor)]
pub struct HillClimber {
    problem: Problem,
    solution: Arc<Solution>,
}

impl Algorithm for HillClimber {
    fn initial_step(&mut self) {
        tracing::info!("Running hill climber");

        let mut visited = Vec::<(i64, i64)>::with_capacity(self.problem.n_steps as usize);
        let (mut at_x, mut at_y) = (self.problem.start_x, self.problem.start_y);
        let mut score = 0;
        let mut world = self.problem.world.clone();

        for _ in 0..self.problem.n_steps {
            // Store the current location and score
            visited.push((at_y, at_x));
            score += world.visit(at_y, at_x).unwrap();

            // Find the maximum value in the 3x3 neighborhood
            let (mut max_value, mut max_dy, mut max_dx) = (0, 1, 1);
            for dy in -1..2 {
                for dx in -1..2 {
                    let value = world.at(at_y + dy, at_x + dx).unwrap_or(-1);

                    if max_value < value {
                        max_value = value;
                        max_dy = dy;
                        max_dx = dx;
                    }
                }
            }

            // If we're not moving, then pick a random direction
            while max_dy == 1 && max_dx == 1 || !world.in_world(at_y + max_dy, at_x + max_dx) {
                let mut rng = rand::rng();
                max_dy = rng.random_range(-1..2);
                max_dx = rng.random_range(-1..2);
            }

            // Move to the maximum value
            at_x += max_dx;
            at_y += max_dy;
        }

        if self.solution.submit_path(&visited, score) {
            tracing::info!("Hill Climber: New high score: {}", score);
        }
    }

    fn next_step(&mut self) {
        thread::sleep(Duration::from_millis(1100));
    }
}
