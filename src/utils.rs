use crate::constants::*;

fn sigma_1(x: f64, a: f64) -> f64 {
    1.0 / (1.0 + ((-(x - a) * 4.0) / ALPHA).exp())
}

fn sigma_2(x: f64, a: f64, b: f64) -> f64 {
    sigma_1(x, a) * (1.0 - sigma_1(x, b))
}

fn sigma_m(x: f64, y: f64, m: f64) -> f64 {
    x * (1.0 - sigma_1(m, 0.5)) + y * sigma_1(m, 0.5)
}

pub fn s(n: f64, m: f64) -> f64 {
    sigma_2(n, sigma_m(B1, D1, m), sigma_m(B2, D2, m))
}

pub fn clamp(x: &mut f64, l: f64, h: f64) {
    if *x < l { *x = l; }
    if *x > h { *x = h; }
}
