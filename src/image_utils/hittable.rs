use std::sync::Arc;

use crate::{
    image_utils::{
        material::{DefaultMat, Material},
        ray::Ray,
    },
    math::{
        interval::Interval,
        vec3::{Point3, Vec3},
        vec3_ops::dot,
    },
};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            outward_normal.negate()
        };
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Point3::default(),
            normal: Vec3::default(),
            mat: Arc::new(DefaultMat),
            t: 0.0,
            front_face: false,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool;
}
