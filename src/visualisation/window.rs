use std::{sync::Arc, thread};

use derive_more::Constructor;
use minifb::{Key, Window, WindowOptions};

use crate::{
    data_structures::{problem::Problem, solution::Solution},
    world::World,
};

#[derive(Constructor)]
pub struct MyWindow {
    problem: Problem,
    solution: Arc<Solution>,
}

impl MyWindow {
    const MAX_WIDTH: usize = 1000;
    const MAX_HEIGHT: usize = 1000;

    pub fn run(self, fps: usize) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            self.run_window(fps);
        })
    }

    fn run_window(&self, fps: usize) {
        let width = self.problem.world.get_grid_size();
        let height = self.problem.world.get_grid_size();

        if width > Self::MAX_WIDTH || height > Self::MAX_HEIGHT {
            tracing::error!("Grid too large for visualizer");
            return;
        }

        let scaling_factor: u64 = f64::min(
            Self::MAX_WIDTH as f64 / width as f64,
            Self::MAX_HEIGHT as f64 / height as f64,
        )
        .floor() as u64;

        let scaling_factor: minifb::Scale = match scaling_factor {
            n if 32 < n => minifb::Scale::X32,
            n if 16 < n => minifb::Scale::X16,
            n if 8 < n => minifb::Scale::X8,
            n if 4 < n => minifb::Scale::X4,
            n if 2 < n => minifb::Scale::X2,
            _ => minifb::Scale::X1,
        };

        // Create a window
        let mut window = Window::new(
            "Visualizer",
            width,
            height,
            WindowOptions {
                scale: scaling_factor,
                ..WindowOptions::default()
            },
        )
        .unwrap();

        window.set_target_fps(fps);

        // Prepare a buffer to store pixel data (ARGB or XRGB)
        let mut buffer: Vec<u32> = vec![0; width * height];

        // Continuously update the window
        while window.is_open() && !window.is_key_down(Key::Escape) {
            // Update the buffer for the grid
            for y in 0..height {
                for x in 0..width {
                    let value = self.problem.world.at(y as i64, x as i64).unwrap() as f64;
                    let value_norm = (value - World::LOWER_BOUND as f64)
                        / ((World::UPPER_BOUND - World::LOWER_BOUND) as f64);

                    // 8 bit RGB [0..255] Grayscale
                    let r: u32 = ((value_norm * 255.0) as u32) << 16;
                    let g: u32 = ((value_norm * 255.0) as u32) << 8;
                    let b: u32 = (value_norm * 255.0) as u32;
                    let rgb = r | g | b;
                    buffer[y * width + x] = rgb;
                }
            }

            /* Plot all paths that are currently in progress */
            let paths_in_progress = self.solution.paths_in_progress.lock().unwrap();
            for (color, path) in paths_in_progress.iter() {
                // Create a color from the hash of the name
                for &(y, x) in path.iter() {
                    buffer[y as usize * width + x as usize] = color.0;
                }
            }

            /* Plot the current best path */
            let path = self.solution.path.lock().unwrap();
            if 0 < path.len() {
                // Plot the entire path in blue
                for &(y, x) in path.iter() {
                    buffer[y as usize * width + x as usize] = 0x0000FF;
                }
                // Plot the beginning in green
                let (y, x) = path[0];
                buffer[y as usize * width + x as usize] = 0x00FF00;
                // Plot the end in red
                let (y, x) = path[path.len() - 1];
                buffer[y as usize * width + x as usize] = 0xFF0000;
            }

            // Render the updated buffer
            window.update_with_buffer(&buffer, width, height).unwrap();
        }
    }
}
