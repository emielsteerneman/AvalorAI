use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicI64, Ordering},
        Mutex,
    },
};

use super::colors::Color;

/// This struct is used to store the best solution found so far
/// It is also used to store the current progress of any algorithm
/// The struct is thread safe, and mulitple algorithms can access it concurrently
#[derive(Default)]
pub struct Solution {
    pub path: Mutex<Vec<(i64, i64)>>,
    pub score: AtomicI64,
    pub paths_in_progress: Mutex<HashMap<Color, Vec<(i64, i64)>>>,
}

impl Solution {
    pub fn get_score(&self) -> i64 {
        self.score.load(Ordering::SeqCst)
    }

    pub fn set_score(&self, score: i64) {
        self.score.store(score, Ordering::SeqCst);
    }

    pub fn submit_path(&self, path: &Vec<(i64, i64)>, score: i64) -> bool {
        let new_highscore = self.get_score() <= score;
        if new_highscore {
            let mut data = self.path.lock().unwrap();
            *data = path.clone();
            self.set_score(score);
        }
        new_highscore
    }

    pub fn submit_path_in_progress(&self, color: Color, path: &Vec<(i64, i64)>) {
        let mut data = self.paths_in_progress.lock().unwrap();
        data.insert(color, path.clone());
    }
}
