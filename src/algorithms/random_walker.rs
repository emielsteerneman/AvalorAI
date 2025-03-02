use derive_more::Constructor;
use std::{sync::Arc, thread, time::Duration};

use crate::data_structures::{colors::Color, problem::Problem, solution::Solution};

use super::Algorithm;

#[derive(Constructor)]
pub struct RandomWalker {
    problem: Problem,
    solution: Arc<Solution>,
}

impl Algorithm for RandomWalker {
    fn initial_step(&mut self) {
        self.next_step();
    }

    fn next_step(&mut self) {
        let mut visited = Vec::<(i64, i64)>::with_capacity(self.problem.n_steps as usize);
        let (mut at_x, mut at_y) = (self.problem.start_x, self.problem.start_y);
        let mut score = 0;
        let mut world = self.problem.world.clone();

        for _ in 0..self.problem.n_steps {
            visited.push((at_y, at_x));
            score += world.visit(at_y, at_x).unwrap();

            loop {
                let dx = rand::random_range(-1..2);
                let dy = rand::random_range(-1..2);
                if world.in_world(at_y + dy, at_x + dx) {
                    at_x += dx;
                    at_y += dy;
                    break;
                }
            }

            // Store the current path
            visited.push((at_y as i64, at_x as i64));
            self.solution
                .submit_path_in_progress(Color::MAGENTA, &visited);
            thread::sleep(Duration::from_millis(1));
        }

        if self.solution.submit_path(&visited, score) {
            tracing::info!("Random Walker: New high score: {}", score);
        }
    }
}
