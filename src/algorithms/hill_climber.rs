use crate::data_structures::{colors::Color, problem::Problem, solution::Solution};
use derive_more::Constructor;
use rand::Rng;
use std::{sync::Arc, thread, time::Duration};

use super::Algorithm;

#[derive(Constructor)]
pub struct HillClimber {
    problem: Problem,
    solution: Arc<Solution>,
}

/// This struct implements the Hill Climber algorithm. It is a simple algorithm that moves in the direction of the highest
/// value in the 3x3 neighborhood. If there is no higher value, it moves in a random direction. Additionally, 5% of the time,
/// it moves in a random direction
impl Algorithm for HillClimber {
    fn next_step(&mut self) {
        let mut visited = Vec::<(i64, i64)>::with_capacity(self.problem.n_steps as usize);
        let (mut at_x, mut at_y) = (self.problem.start_x, self.problem.start_y);
        let mut score = 0;
        let mut world = self.problem.world.clone();

        for _ in 0..self.problem.n_steps {
            // Store the current location and score
            visited.push((at_y, at_x));
            score += world.visit(at_y, at_x).unwrap();

            // Find the maximum value in the 3x3 neighborhood
            let (mut max_value, mut max_dy, mut max_dx) = (0, 0, 0);
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

            // Take a random step 5% of the time
            let mut take_random_step = rand::random_range(0. ..1.) < 0.05;

            // If we're not moving, then pick a random direction
            while max_dy == 0 && max_dx == 0
                || !world.in_world(at_y + max_dy, at_x + max_dx)
                || take_random_step
            {
                take_random_step = false;
                let mut rng = rand::rng();
                max_dy = rng.random_range(-1..2);
                max_dx = rng.random_range(-1..2);
            }

            // Move to the maximum value
            at_x += max_dx;
            at_y += max_dy;

            // Store the current path
            visited.push((at_y as i64, at_x as i64));
            self.solution
                .submit_path_in_progress(Color::YELLOW, &visited);
            thread::sleep(Duration::from_millis(1));
        }

        if self.solution.submit_path(&visited, score) {
            tracing::info!("Hill Climber: New high score: {}", score);
        }
    }
}
