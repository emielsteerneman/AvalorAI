use crate::data_structures::{colors::Color, problem::Problem, solution::Solution};
use derive_more::Constructor;
use ndarray::{s, Array2, ArrayView2, Axis};
use std::{sync::Arc, thread, time::Duration};

use super::Algorithm;

/// This struct is used to implement the Hill Climber Guided algorithm. It is a modified version of the Hill Climber algorithm.
/// It searching in all direction within a certain range, and moves in the direction of a strong pull. If there is no strong pull,
/// the search range is increased. If the search range is too large, the algorithm stops. At this point, the algorithm will walk
/// in a random direction. Additionally, 5% of the time, it moves in a random direction
#[derive(Constructor)]
pub struct HillClimberGuided {
    problem: Problem,
    solution: Arc<Solution>,
}

impl Algorithm for HillClimberGuided {
    fn next_step(&mut self) {
        let mut visited = Vec::<(i64, i64)>::with_capacity(self.problem.n_steps as usize);
        let (mut at_x, mut at_y) = (self.problem.start_x, self.problem.start_y);

        // Keep track of the current score
        let mut score = 0;

        // Run the algorithm for n_steps
        for _ in 0..self.problem.n_steps {
            // Store the current location and score
            visited.push((at_y, at_x));
            score += self.problem.world.visit(at_y, at_x).unwrap();

            // Search granularity. This will keep growing
            let mut search_granularity = 1;
            // Search ranges matching the granularity. These will keep growing
            let mut search_ranges = vec![-1, 0, 1];

            // Loop until we find a pull towards a certain direction, or until its decided that no pull can be found
            loop {
                // Initialize a 3x3 grid that will store the values of the 9 blocks around the current location
                let mut grid3x3: Array2<i64> = Array2::zeros((3, 3));

                // Fill the 3x3 grid
                for dy in 0..3 as i64 {
                    for dx in 0..3 as i64 {
                        // Skip the center. Not moving is not an option
                        if dx == 1 && dy == 1 {
                            continue;
                        }

                        // Get the topleft corner of the block
                        let y = at_y + search_ranges[dy as usize];
                        let x = at_x + search_ranges[dx as usize];

                        // Get the block (a submatrix of the world)
                        let world_view = self.problem.world.get_information();
                        // tracing::info!("y {}, y+S {}, x {}, x+S {}", y, y+S, x, x+S);
                        let submatrix = get_submatrix(
                            &world_view,
                            y,
                            y + search_granularity,
                            x,
                            x + search_granularity,
                        );

                        // If a block was found, store the summation of that block
                        // (it might happen that the block is completely of bounds)
                        if submatrix.is_some() {
                            let submatrix = submatrix.unwrap();
                            grid3x3[[dy as usize, dx as usize]] = submatrix.sum();
                        }
                    }
                }

                /* Determine termination conditions */

                // Determine if there is a pull towards a certain direction
                let (step_taken, next_y, next_x) = match get_pull(grid3x3.mapv(|v| v as f64), 0.4) {
                    // Pulls towards the center are not valid
                    Some((0, 0)) => (false, 0, 0),
                    // Pulls towards another direction are valid
                    Some((dy, dx)) => (true, at_y + dy, at_x + dx),
                    // No pull was found
                    _ => (false, 0, 0),
                };

                // Determine if the search ranges are too large
                let search_too_large = (self.problem.world.get_grid_size() as i64)
                    < (search_ranges[1] - search_ranges[0]);

                // Take a random step 5% of the time
                let take_random_step = rand::random_range(0. ..1.) < 0.05;

                // Update search ranges
                search_ranges[0] -= search_granularity * 3;
                search_ranges[1] -= search_granularity;
                search_ranges[2] += search_granularity;
                // Update search granularity
                search_granularity *= 3;

                if (!step_taken && search_too_large) || take_random_step {
                    // Search for a random step that can be taken
                    loop {
                        let dx = rand::random_range(-1..2);
                        let dy = rand::random_range(-1..2);
                        if self.problem.world.in_world(at_y + dy, at_x + dx) {
                            at_x += dx;
                            at_y += dy;
                            break;
                        }
                    }
                    break;
                } else if step_taken {
                    at_y = next_y;
                    at_x = next_x;
                    break;
                }
            }

            // Store the current path
            visited.push((at_y as i64, at_x as i64));
            self.solution.submit_path_in_progress(Color::CYAN, &visited);
            thread::sleep(Duration::from_millis(1));
        }

        if self.solution.submit_path(&visited, score) {
            tracing::info!("Hill Climber Guided: New high score: {}", score);
        }
    }
}

fn get_submatrix<'a>(
    matrix: &'a ArrayView2<i64>,
    start_row: i64,
    end_row: i64,
    start_col: i64,
    end_col: i64,
) -> Option<ArrayView2<'a, i64>> {
    let rows = matrix.nrows() as i64;
    let cols = matrix.ncols() as i64;

    // Check if the submatrix is within bounds and valid
    if rows < start_row
        || cols < start_col
        || end_row < 0
        || end_col < 0
        || end_row < start_row
        || end_col < start_col
    {
        return None;
    }

    // Clamp the bounds to avoid out-of-bounds slicing
    let start_row_clamped = start_row.max(0) as usize;
    let start_col_clamped = start_col.max(0) as usize;
    let end_row_clamped = end_row.min(rows) as usize;
    let end_col_clamped = end_col.min(cols) as usize;

    Some(matrix.slice(s![
        start_row_clamped..end_row_clamped,
        start_col_clamped..end_col_clamped
    ]))
}

/// This function returns the direction of the pull, given that the center is pulled towards one of the directions around it
/// A pull is only considered strong enough if the given threshold is reached. If no pull is strong enough, None is returned
fn get_pull(grid: Array2<f64>, threshold: f64) -> Option<(i64, i64)> {
    // Increment all values by 0.001 to avoid division by 0
    let grid_sum = grid.sum() + 0.001;
    let col_sum = grid.sum_axis(Axis(0)) / grid_sum;
    let row_sum = grid.sum_axis(Axis(1)) / grid_sum;
    // Find the first value that has a strong enough pull
    let pull_x = col_sum.iter().position(|&v| threshold < v);
    let pull_y = row_sum.iter().position(|&v| threshold < v);

    // If there is a strong pull both horizontally and vertically, return the direction
    match (pull_y, pull_x) {
        (Some(pull_y), Some(pull_x)) => Some((pull_y as i64 - 1, pull_x as i64 - 1)),
        _ => None,
    }
}
