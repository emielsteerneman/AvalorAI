use std::sync::{
    atomic::{AtomicI64, Ordering},
    Mutex,
};

#[derive(Default)]
pub struct Solution {
    pub path: Mutex<Vec<(i64, i64)>>,
    pub score: AtomicI64,
}

impl Solution {
    pub fn get_score(&self) -> i64 {
        self.score.load(Ordering::SeqCst)
    }

    pub fn set_score(&self, score: i64) {
        self.score.store(score, Ordering::SeqCst);
    }

    pub fn submit_path(&self, path: &Vec<(i64, i64)>, score: i64) -> bool {
        let new_highscore = self.get_score() < score;
        if new_highscore {
            let mut data = self.path.lock().unwrap();
            *data = path.clone();
            self.set_score(score);
        }
        new_highscore
    }
}
