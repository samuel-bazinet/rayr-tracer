use crate::{
    image_utils::ray::Ray,
    math::{
        interval::Interval,
        vec3::{Point3, Vec3},
        vec3_ops::dot,
    },
};

#[derive(Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            let mut normal = *outward_normal;
            normal.negate();
            normal
        };
    }

    pub fn copy_from(&mut self, other: &mut Self) {
        self.p = other.p;
        self.normal = other.normal;
        self.t = other.t;
        self.front_face = other.front_face;
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool;
}
