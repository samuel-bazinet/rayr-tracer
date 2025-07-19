use crate::math::{vec3::Point3, vec3_ops::dot};
use ray::Ray;

pub mod camera;
pub mod colour;
pub mod hittable;
pub mod hittable_list;
pub mod material;
pub mod ray;

pub fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = center - ray.origin();
    let a = dot(ray.direction(), ray.direction());
    let h = dot(ray.direction(), &oc);
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = h * h - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (h - f64::sqrt(discriminant)) / (a)
    }
}
