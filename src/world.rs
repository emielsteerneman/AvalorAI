use rand::Rng;

#[derive(Clone, Debug)]
pub struct World {
    // 2d float vector
    information: Vec<Vec<i64>>,
}

impl World {
    pub fn new(n: usize) -> World {
        let information = vec![vec![0 as i64; n]; n];

        World { information }
    }

    pub fn random(n: usize, sparsity: f64) -> World {
        let mut world = World::new(n);
        world.fill_random(sparsity);
        world
    }

    pub fn fill_random(&mut self, sparsity: f64) {
        let mut rng = rand::rng();

        for y in 0..self.information.len() {
            for x in 0..self.information[y].len() {
                if sparsity < rng.random_range(0.0..1.0) {
                    self.information[y][x] = rng.random_range(0..100);
                }
            }
        }
    }

    pub fn at(&self, y: i64, x: i64) -> Option<i64> {
        if self.in_world(y, x) {
            Some(self.information[y as usize][x as usize])
        } else {
            None
        }
    }

    pub fn set(&mut self, y: i64, x: i64, value: i64) {
        if self.in_world(y, x) {
            self.information[y as usize][x as usize] = value;
        }
    }

    pub fn visit(&mut self, y: i64, x: i64) -> Option<i64> {
        let value = self.at(y, x);
        if value.is_some() {
            self.set(y, x, 0);
        }
        value
    }

    pub fn in_world(&self, y: i64, x: i64) -> bool {
        0 <= y
            && y < self.information.len() as i64
            && 0 <= x
            && x < self.information[0].len() as i64
    }

    pub fn print(&self) {
        for y in 0..self.information.len() {
            for x in 0..self.information[y].len() {
                print!("{:3.0} ", self.information[y][x]);
            }
            println!();
        }
    }
}
