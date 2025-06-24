use crate::{
    image_utils::{colour::Colour, hit_sphere},
    math::{
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

pub fn ray_color(ray: &Ray) -> Colour {
    let s_a = hit_sphere(&Point3::from_vals(0.0, 0.0, -1.0), 0.5, ray);
    let s_b = hit_sphere(&Point3::from_vals(-0.3, 0.0, -1.0), 0.5, ray);
    if s_a > 0.0 {
        let n = unit_vector(&(ray.at(s_a) - Vec3::from_vals(0.0, 0.0, -1.0)));
        Colour::from_vals(n.x() + 1.0, n.y() + 0.0, n.z() + 0.0) * 0.5
    } else if s_b > 0.0 {
        let n = unit_vector(&(ray.at(s_a) - Vec3::from_vals(0.0, 0.0, -1.0)));
        Colour::from_vals(n.x() + 1.0, n.y() + 0.0, n.z() + 0.0) * 0.5
    } else {
        let unit_direction = unit_vector(ray.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        blended_value(
            a,
            Colour::from_vals(0.8, 1.0, 0.8),
            Colour::from_vals(0.5, 1.0, 0.7),
        )
    }
}

fn blended_value(a: f64, start_value: Colour, end_value: Colour) -> Colour {
    start_value * (1.0 - a) + end_value * a
}
