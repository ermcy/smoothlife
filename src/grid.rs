use std::ops::Mul;

use rand::Rng;

use crate::constants::*;
use crate::utils::{clamp, s};

pub struct Grid {
    pub data: Vec<Vec<f64>>,
    pub height: usize,
    pub width: usize
}

impl Grid {
    pub fn empty(height: usize, width: usize) -> Self {
        return Self {
            data: vec![vec![0.0; width]; height],
            height,
            width,
        }
    }

    pub fn init(height: usize, width: usize, shrink_factor: Option<usize>) -> Self {
        let mut rng = rand::thread_rng();
        let mut result = Grid::empty(height, width);
        match shrink_factor {
            None => {
                for y in result.data.iter_mut() {
                    for x in y.iter_mut() {
                        *x = rng.gen::<f64>();
                    }
                }
            }
            Some(f) => {
                let w = width / f;
                let h = height / f;
                for y in result.data.iter_mut().take(h) {
                    for x in y.iter_mut().take(w) {
                        *x = rng.gen::<f64>();
                    }
                }
            }
        }

        return result;
    }

    pub fn display(&self) {
        for row in &self.data {
            for e in row {
                let idx = e * (LEVEL.len() - 1) as f64;
                let c = LEVEL[idx as usize];
                print!("{c}");
                print!("{c}");
            }
            println!();
        }
    }

    // pub fn as_string(&mut self) -> Vec<u8> {
    //     let mut buffer = vec![];
    //     for y in self.data.iter_mut() {
    //         for e in y.iter_mut() {
    //             let idx = *e * (LEVEL.len() - 1) as f64;
    //             let c = LEVEL[idx as usize];
    //             buffer.push(c as u8);
    //         }
    //         buffer.push(b'\n');
    //     }
    //     return buffer;
    // }

    pub fn compute_diff(&self) -> Self {
        let mut diff_grid = Grid::empty(self.height, self.width);
        for cy in 0..self.height {
            for cx in 0..self.width {
                let mut m = 0.0;
                let mut n = 0.0;
                let mut am = 0.0;
                let mut an = 0.0;
                let ri = RA / 3;
                for dy in -(RA - 1)..=(RA - 1) {
                    for dx in -(RA - 1)..=(RA - 1) {
                        let x = cx.wrapping_add(dx as usize).rem_euclid(self.width);
                        let y = cy.wrapping_add(dy as usize).rem_euclid(self.height);
                        let pow = dx * dx + dy * dy;
                        if pow <= ri * ri {
                            m += self.data[y][x];
                            am += 1.0;
                        } else if pow <= RA * RA {
                            n += self.data[y][x];
                            an += 1.0;
                        }
                    }
                }
                m /= am;
                n /= an;
                let q = s(n, m);
                diff_grid.data[cy][cx] = (2.0 * q) - 1.0;
            }
        }
        return diff_grid;
    }

    pub fn apply_grid_diff(&mut self, mut other: Self, dt: f64) {
        for (g, d) in self.data.iter_mut().zip(other.data.iter_mut()) {
            for (old, new) in g.iter_mut().zip(d.iter_mut()) {
                *old += dt.mul(*new);
                clamp(old, 0.0, 1.0);
            }
        }
    }
}