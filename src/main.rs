use std::ops::Mul;

use rand::Rng;

const HEIGHT: usize = 250;
const WIDTH: usize = 250;

const ALPHA: f64 = 0.030;

const B1: f64 = 0.278;
const B2: f64 = 0.365;
const D1: f64 = 0.267;
const D2: f64 = 0.445;
const RA: i32 = 15;
const LEVEL: [char; 9] = [' ', '.', '-', '=', 'c', 'o', 'a', 'A', '@'];


struct Grid {
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

    pub fn init(height: usize, width: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut result = Grid::empty(height, width);
        for y in result.data.iter_mut() {
            for x in y.iter_mut() {
                *x = rng.gen::<f64>();
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
            }
            println!();
        }
    }

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
                        let x = cx.wrapping_add(dx as usize).rem_euclid(WIDTH);
                        let y = cy.wrapping_add(dy as usize).rem_euclid(HEIGHT);
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

fn sigma_1(x: f64, a: f64) -> f64 {
    1.0 / (1.0 + ((-(x - a) * 4.0) / ALPHA).exp())
}

fn sigma_2(x: f64, a: f64, b: f64) -> f64 {
    sigma_1(x, a) * (1.0 - sigma_1(x, b))
}

fn sigma_m(x: f64, y: f64, m: f64) -> f64 {
    x * (1.0 - sigma_1(m, 0.5)) + y * sigma_1(m, 0.5)
}

fn s(n: f64, m: f64) -> f64 {
    sigma_2(n, sigma_m(B1, D1, m), sigma_m(B2, D2, m))
}

fn clamp(x: &mut f64, l: f64, h: f64) {
    if *x < l { *x = l; }
    if *x > h { *x = h; }
}

fn main() {
    let dt = 0.03;
    let mut grid = Grid::init(HEIGHT, WIDTH);
    grid.display();
    loop {
        let diff = grid.compute_diff();
        grid.apply_grid_diff(diff, dt);
        grid.display();
    }
}
