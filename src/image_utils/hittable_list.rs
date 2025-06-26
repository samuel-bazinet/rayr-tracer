use crate::math::interval::Interval;

use super::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

use std::rc::Rc;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn from_hittable(object: &Rc<dyn Hittable>) -> Self {
        Self {
            objects: vec![object.clone()],
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: &Rc<dyn Hittable>) {
        self.objects.push(object.clone());
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;

        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(
                ray,
                &Interval::from(ray_t.min, closest_so_far),
                &mut temp_rec,
            ) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.copy_from(&mut temp_rec);
            }
        }

        hit_anything
    }
}
