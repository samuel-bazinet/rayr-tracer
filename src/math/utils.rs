use rand::prelude::*;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_f64() -> f64 {
    rand::rng().random_range(0.0..1.0)
}

pub fn random_f64_bounded(min: f64, max: f64) -> f64 {
    rand::rng().random_range(min..max)
}
