use std::sync::Arc;

use derive_more::Constructor;

use crate::data_structures::{problem::Problem, solution::Solution};

use super::Algorithm;

#[derive(Constructor)]
pub struct RandomImprover {
    problem: Problem,
    solution: Arc<Solution>,
}

// impl Algorithm for RandomImprover {

//     fn next_step(&mut self) {
//         let current_path = self.solution.path.lock().unwrap();
//         let current_score = self.solution.get_score();

//         let mut world = self.problem.world.clone();
//         let cumulative_score = current_path.iter().scan(0, |score, &(y, x)| {
//             *score += world.visit(y, x).unwrap();
//             Some(*score)
//         });


//     }

// }