use crate::{
    image_utils::{colour::Colour, hittable::HitRecord, material::Material, ray::Ray},
    math::vec3_ops::random_unit_vector,
};

pub struct Lambertian {
    albedo: Colour,
}

impl Lambertian {
    pub fn from(albedo: &Colour) -> Self {
        Self { albedo: *albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::from(&rec.p, &scatter_direction);
        *attenuation = self.albedo;
        true
    }
}
