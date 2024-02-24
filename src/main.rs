use std::io::{BufWriter, stdout, Stdout, Write};
use std::thread;

use rand::Rng;

const WIDTH: usize = 250;
const HEIGHT: usize = 250;

const ALPHA: f64 = 0.028;

const B1: f64 = 0.278;
const B2: f64 = 0.365;
const D1: f64 = 0.267;
const D2: f64 = 0.445;
const RA: i32 = 15;
const LEVEL: [char; 9] = [' ', '.', '-', '=', 'c', 'o', 'a', 'A', '@'];

type Erm = [[f64; HEIGHT]; WIDTH];

struct Grid(Erm);

impl Grid {
    pub const fn empty() -> Self {
        Self([[0.0; HEIGHT]; WIDTH])
    }

    pub fn init() -> Self {
        let mut rng = rand::thread_rng();
        let mut grid = [[0.0; HEIGHT]; WIDTH];
        for sub_arr in grid.iter_mut() {
            for elem in sub_arr.iter_mut() {
                *elem = rng.gen_range(0.0..1.0);
            }
        }
        Self(grid)
    }

    pub fn display(&self, buf_writer: &mut BufWriter<Stdout>) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let idx = self.0[y][x] * (LEVEL.len() - 1) as f64;
                let c = LEVEL[idx as usize];
                if let Err(why) = buf_writer.write_all(&[c as u8]) {
                    unreachable!("{:#?}", why);
                };
            }
            if let Err(why) = buf_writer.write_all(&[b'\n']) {
                unreachable!("{:#?}", why);
            };
        }
    }

    pub fn compute_diff(&self) -> Self {
        let mut diff_grid = Grid::empty();
        for cy in 0..HEIGHT {
            for cx in 0..WIDTH {
                let mut m = 0.0;
                let mut n = 0.0;
                let mut am = 0.0;
                let mut an = 0.0;
                let ri = RA / 3;
                for dy in -(RA - 1)..=(RA - 1) {
                    for dx in -(RA - 1)..=(RA - 1) {
                        let x = (cx as i32 + dx).rem_euclid(WIDTH as i32) as usize;
                        let y = (cy as i32 + dy).rem_euclid(HEIGHT as i32) as usize;
                        let pow = dx * dx + dy * dy;
                        if pow <= ri * ri {
                            m += self.0[y][x];
                            am += 1.0;
                        }
                        if pow <= RA * RA {
                            n += self.0[y][x];
                            an += 1.0;
                        }
                    }
                }
                m /= am;
                n /= an;
                let q = s(n, m);
                diff_grid.0[cy][cx] = (2.0 * q) - 1.0;
            }
        }
        diff_grid
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
    let thread = match thread::Builder::new().stack_size(10000000000).spawn(|| {
        let stdout = stdout();
        let mut buff_out = BufWriter::new(stdout);
        let dt = 0.05;
        let mut grid = Grid::init();
        grid.display(&mut buff_out);
        loop {
            let diff = grid.compute_diff();
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    grid.0[y][x] += dt * diff.0[y][x];
                    clamp(&mut grid.0[y][x], 0.0, 1.0);
                }
            }
            grid.display(&mut buff_out);
        }
    }) {
        Ok(ok) => ok,
        Err(why) => unreachable!("{:#?}", why)
    };
    if let Err(why) = thread.join() {
        unreachable!("{:#?}", why);
    }
}
