use crate::math::{vec3::Point3, vec3_ops::dot};
use ray::Ray;

pub mod camera;
pub mod colour;
pub mod hittable;
pub mod hittable_list;
pub mod material;
pub mod ray;

pub fn calculate_viewport(aspect_ratio: f64, image_width: u32) -> (f64, f64) {
    let image_height = if image_width as f64 > aspect_ratio {
        (image_width as f64 / aspect_ratio).floor() as u32
    } else {
        1
    };

    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    (viewport_height, viewport_width)
}

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
