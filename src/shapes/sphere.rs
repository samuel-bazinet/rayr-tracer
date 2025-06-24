use crate::{
    image_utils::{
        hittable::{HitRecord, Hittable},
        ray::Ray,
    },
    math::{vec3::Point3, vec3_ops::dot},
};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn from(center: &Point3, radius: f64) -> Self {
        Self {
            center: *center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let oc = self.center - ray.origin();
        let a = dot(ray.direction(), ray.direction());
        let h = dot(ray.direction(), &oc);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = f64::sqrt(discriminant);
        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h - sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }
        rec.t = root;
        rec.p = ray.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        true
    }
}
