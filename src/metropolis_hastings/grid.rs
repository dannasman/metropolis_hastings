use rand::prelude::*;
use std::fs::File;
use std::io::{Error, Write};

#[derive(Debug, Clone)]
pub struct SpinGrid {
    dim_x: usize,
    dim_y: usize,
    pub grid: Vec<Vec<i32>>, //2D spin grid
}

impl SpinGrid {
    pub fn new(dim_x: usize, dim_y: usize) -> Self {
        SpinGrid {
            dim_x,
            dim_y,
            grid: vec![vec![1; dim_y]; dim_x], //initialize grid with all spins up
        }
    }

    pub fn calculate_configurations(&mut self, inter_strength: f64, temperature: f64) {
        let mut rng = rand::thread_rng();

        let n = (self.dim_x) * (self.dim_y);
        for _i in 0..n {
            let x: usize = rng.gen_range(0..self.dim_x);
            let y: usize = rng.gen_range(0..self.dim_y);
            let sigma_xy = self.grid[x][y] as f64;
            let mut sum = 0;

            if x < self.dim_x - 1 {
                sum += self.grid[x + 1][y];
            }

            if x > 0 {
                sum += self.grid[x - 1][y];
            }

            if y < self.dim_y - 1 {
                sum += self.grid[x][y + 1];
            }

            if y > 0 {
                sum += self.grid[x][y - 1];
            }

            let s = sum as f64;
            let energy = 2.0 * inter_strength * sigma_xy * s;

            let rn: f64 = rng.gen();
            if energy < 0.0 || rn < (-energy / temperature).exp() {
                self.grid[x][y] = -self.grid[x][y];
            }
        }
    }

    pub fn save(&mut self, filename: &str) -> Result<(), Error> {
        let mut create = File::create(filename)?;

        writeln!(create, "x y spin")?;
        for i in 0..self.dim_x {
            for j in 0..self.dim_y {
                writeln!(create, "{} {} {}", i, j, self.grid[i][j])?;
            }
        }
        Ok(())
    }
}
