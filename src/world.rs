use ndarray::{Array2, ArrayView2, Zip};
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};
use rand::Rng;
use statrs::{
    distribution::{ContinuousCDF, Normal},
    statistics::Statistics,
};

/// A world is represented by a 2D array of integers
#[derive(Clone)]
pub struct World {
    grid_size: usize,
    information: Array2<i64>,
}

#[allow(dead_code)]
impl World {
    pub const LOWER_BOUND: i64 = 0;
    pub const UPPER_BOUND: i64 = 100;

    pub fn new(grid_size: usize) -> World {
        let information = Array2::zeros((grid_size, grid_size));
        World {
            grid_size,
            information,
        }
    }

    /// Creates a new world with random values
    pub fn random(grid_size: usize, sparsity: f64) -> World {
        let mut world = World::new(grid_size);
        world.fill_random(sparsity);
        world
    }

    /// Creates a new world with perlin noise
    pub fn perlin(grid_size: usize, sparsity: f64) -> World {
        let mut world = World::new(grid_size);
        world.fill_perlin(sparsity);
        world
    }

    /// Fills the world with random values
    /// sparsity: 0.0 - 1.0
    pub fn fill_random(&mut self, sparsity: f64) {
        let mut rng = rand::rng();

        for val in self.information.iter_mut() {
            if sparsity < rng.random_range(0.0..1.0) {
                *val = rng.random_range(Self::LOWER_BOUND..Self::UPPER_BOUND);
            }
        }
    }

    /// Fills the world with perlin noise
    pub fn fill_perlin(&mut self, sparsity: f64) {
        let seed = rand::rng().random_range(0..u32::MAX);
        let fmb = Fbm::<Perlin>::new(seed)
            .set_octaves(4)
            .set_frequency(2.0)
            .set_lacunarity(5.0)
            .set_persistence(1.);

        let grid_size_f64 = self.grid_size as f64;
        Zip::indexed(&mut self.information).for_each(|(y, x), val| {
            let noise_val = fmb.get([x as f64 / grid_size_f64, y as f64 / grid_size_f64]);
            let value_norm = (Self::UPPER_BOUND as f64) * (noise_val + 1.0) / 2.0;
            *val = value_norm as i64;
        });

        if sparsity <= 0.0 {
            return;
        }

        // Flatten to f64 for stats
        let flattened = self.information.mapv(|v| v as f64);

        // Mean & standard deviation (population or sample depends on your use case)
        let mean = flattened.view().mean();
        let std = flattened.view().std(0.0); // 0.0 = population, 1.0 = sample

        tracing::info!("Mean: {}", mean);
        tracing::info!("Std dev: {}", std);

        // Compute z-score cutoff from Normal distribution
        let zscore = Normal::new(mean, std).unwrap().inverse_cdf(sparsity) as i64;

        let max_value: i64 = flattened.max() as i64 - zscore;
        tracing::info!("Zscore: {}", zscore);

        // Shift and rescale all values
        Zip::from(&mut self.information).for_each(|val| {
            let shifted = i64::max(0, *val - zscore);
            let scaled = (Self::UPPER_BOUND as f64) * (shifted as f64) / (max_value as f64);
            *val = scaled as i64;
        });
    }

    pub fn at(&self, y: i64, x: i64) -> Option<i64> {
        if self.in_world(y, x) {
            Some(self.information[[y as usize, x as usize]])
        } else {
            None
        }
    }

    pub fn set(&mut self, y: i64, x: i64, value: i64) {
        if self.in_world(y, x) {
            self.information[[y as usize, x as usize]] = value;
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
        0 <= y && y < self.grid_size as i64 && 0 <= x && x < self.grid_size as i64
    }

    pub fn get_grid_size(&self) -> usize {
        self.grid_size
    }

    pub fn get_information(&self) -> ArrayView2<i64> {
        self.information.view()
    }
}
