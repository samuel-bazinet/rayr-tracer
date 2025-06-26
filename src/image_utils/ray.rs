use crate::{
    constants::INFINITY,
    image_utils::{
        colour::Colour,
        hittable::{HitRecord, Hittable},
        hittable_list::HittableList,
    },
    math::{
        interval::Interval,
        vec3::{Point3, Vec3},
        vec3_ops::unit_vector,
    },
};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new() -> Self {
        Self {
            orig: Point3::default(),
            dir: Vec3::default(),
        }
    }

    pub fn from(orig: &Point3, dir: &Vec3) -> Self {
        Self {
            orig: *orig,
            dir: *dir,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self::new()
    }
}

pub fn ray_color(ray: &Ray, world: &HittableList) -> Colour {
    let mut rec = HitRecord::default();
    if world.hit(ray, &Interval::from(0.0, INFINITY), &mut rec) {
        (rec.normal + Colour::from(1.0, 1.0, 1.0)) * 0.5
    } else {
        let unit_direction = unit_vector(ray.direction());
        let a = (unit_direction.y() + 1.0) * 0.5;
        blended_value(a, Colour::from(1.0, 1.0, 1.0), Colour::from(0.5, 0.7, 1.0))
    }
}

fn blended_value(a: f64, start_value: Colour, end_value: Colour) -> Colour {
    start_value * (1.0 - a) + end_value * a
}
