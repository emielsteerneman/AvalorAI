use noise::{core::value, Fbm, MultiFractal, NoiseFn, Perlin};
use rand::Rng;
use statrs::{distribution::{ContinuousCDF, Normal}, statistics::Statistics};

#[derive(Clone, Debug)]
pub struct World {
    grid_size: usize,
    information: Vec<Vec<i64>>,
}

impl World {

    pub const LOWER_BOUND: i64 = 0;
    pub const UPPER_BOUND: i64 = 100;

    pub fn new(grid_size: usize) -> World {
        let information = vec![vec![0 as i64; grid_size]; grid_size];

        World { 
            grid_size,
            information 
        }
    }

    pub fn random(grid_size: usize, sparsity: f64) -> World {
        let mut world = World::new(grid_size);
        world.fill_random(sparsity);
        world
    }

    pub fn perlin(grid_size: usize, sparsity: f64) -> World {
        let mut world = World::new(grid_size);
        world.fill_perlin(sparsity);
        world
    }

    pub fn fill_random(&mut self, sparsity: f64) {
        let mut rng = rand::rng();

        for y in 0..self.grid_size {
            for x in 0..self.grid_size {
                if sparsity < rng.random_range(0.0..1.0) {
                    self.information[y][x] = rng.random_range(Self::LOWER_BOUND..Self::UPPER_BOUND);
                }
            }
        }
    }

    pub fn fill_perlin(&mut self, sparsity: f64) {
        
        let fmb = Fbm::<Perlin>::new(0)
        .set_octaves(4)
        .set_frequency(2.0)
        .set_lacunarity(5.0)
        .set_persistence(1.);
        
        let grid_size_f64 = self.grid_size as f64;
        for y in 0..self.grid_size {
            for x in 0..self.grid_size {
                let value = fmb.get([x as f64 / grid_size_f64, y as f64 / grid_size_f64]);
                let value_norm = (Self::UPPER_BOUND as f64) * (value + 1.0) / 2.0;
                self.information[y][x] = value_norm as i64;
            }
        }

        if sparsity == 0.0 {
            return
        }

        let flattened = self.information
            .iter()
            .flat_map(|row| row.iter().map(|&val| val as f64))
            .collect::<Vec<f64>>();
        let mean = flattened.as_slice().mean();
        let std = flattened.as_slice().std_dev();
        
        tracing::info!("Mean: {}", mean);
        tracing::info!("Std dev: {}", std);

        // percentage to std dev
        let zscore = Normal::new(mean, std).unwrap().inverse_cdf(sparsity) as i64;
        let max_value:i64 = flattened.as_slice().max() as i64 - zscore;
        tracing::info!("Zscore: {}", zscore);

        for y in 0..self.grid_size {
            for x in 0..self.grid_size {
                let value = i64::max(0, self.information[y][x] - zscore);
                let value_norm = (Self::UPPER_BOUND as f64) * (value as f64) / max_value as f64;
                self.information[y][x] = value_norm as i64;
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

    pub fn get_grid_size(&self) -> usize {
        self.grid_size
    }
}
