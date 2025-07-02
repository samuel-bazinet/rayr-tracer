use crate::image_utils::{colour::Colour, hittable::HitRecord, ray::Ray};

pub mod dielectric;
pub mod lambertian;
pub mod metal;

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct DefaultMat;

impl Material for DefaultMat {
    fn scatter(
        &self,
        _ray_in: &Ray,
        _hit_record: &HitRecord,
        _attenuation: &mut Colour,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
}
