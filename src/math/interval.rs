use crate::math::utils::INFINITY;

#[derive(Default)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const fn new() -> Self {
        Self {
            min: -INFINITY,
            max: INFINITY,
        }
    }

    pub const fn from(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}

pub const EMPTY: Interval = Interval::from(INFINITY, -INFINITY);
pub const UNIVERSE: Interval = Interval::from(-INFINITY, INFINITY);
