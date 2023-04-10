use rand::prelude::*;
use std::fs::File;
use std::io::{Error, Write};

#[derive(Debug, Clone)]
pub struct SpinGrid {
    dim_x: usize,
    dim_y: usize,
    order: Option<usize>,
    pub grid: Vec<Vec<i32>>, //2D spin grid
}

impl SpinGrid {
    pub fn new(dim_x: usize, dim_y: usize) -> Self {
        SpinGrid {
            dim_x,
            dim_y,
            order: None,
            grid: vec![vec![1; dim_y]; dim_x], //initialize grid with all spins up
        }
    }

    pub fn new_hcurve(order: usize) -> Self {
        let dim = 1 << order;
        SpinGrid {
            dim_x: dim,
            dim_y: dim,
            order: Some(order),
            grid: vec![vec![1; dim]; dim],
        }
    }

    pub fn calculate_configurations(&mut self, inter_strength: f64, temperature: f64) {
        let mut rng = rand::thread_rng();

        let n = self.dim_x * self.dim_y;
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

    pub fn calculate_configurations_hcurve(
        &mut self,
        inter_strength: f64,
        temperature: f64,
    ) -> Result<(), &'static str> {
        let mut rng = rand::thread_rng();

        //parallel prefix method is used to get (x, y) from Hilbert curve distance s
        if let Some(n) = self.order {
            let max_length = (1 << n) * (1 << n);
            for i in 0..max_length {
                let mut s = i | (0x55555555 << (2 * n));
                let sr = (s >> 1) & 0x55555555;
                let mut cs = ((s & 0x55555555) + sr) ^ 0x55555555;

                cs = cs ^ (cs >> 2);
                cs = cs ^ (cs >> 4);
                cs = cs ^ (cs >> 8);
                cs = cs ^ (cs >> 16);
                let swap = cs & 0x55555555;
                let comp = (cs >> 1) & 0x55555555;

                let mut t = (s & swap) ^ comp;
                s = s ^ sr ^ t ^ (t << 1);

                s &= (1 << (2 * n)) - 1;

                t = (s ^ (s >> 1)) & 0x22222222;
                s = s ^ t ^ (t << 1);

                t = (s ^ (s >> 2)) & 0x0C0C0C0C;
                s = s ^ t ^ (t << 2);

                t = (s ^ (s >> 4)) & 0x00F000F0;
                s = s ^ t ^ (t << 4);

                t = (s ^ (s >> 8)) & 0x0000FF00;
                s = s ^ t ^ (t << 8);

                let x = s >> 16;
                let y = s & 0xFFFF;
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
            Ok(())
        } else {
            Err("to calculate configuration with Hilbert curve use SpinGrid::new_hcurve(order) when creating the grid")
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
