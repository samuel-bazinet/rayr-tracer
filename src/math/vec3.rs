use std::fmt::Display;

use crate::math::utils::{random_f64, random_f64_bounded};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub e: [f64; 3],
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new() -> Self {
        Self { e: [0f64; 3] }
    }

    pub fn from(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn random() -> Self {
        Self {
            e: [random_f64(), random_f64(), random_f64()],
        }
    }

    pub fn random_bounded(min: f64, max: f64) -> Self {
        Self {
            e: [
                random_f64_bounded(min, max),
                random_f64_bounded(min, max),
                random_f64_bounded(min, max),
            ],
        }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn negate(&self) -> Self {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }

    pub fn get_element(&self, index: usize) -> f64 {
        self.e[index]
    }

    pub fn get_element_ref(&self, index: usize) -> &f64 {
        &self.e[index]
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::new()
    }
}
