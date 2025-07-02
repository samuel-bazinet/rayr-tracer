use crate::{
    image_utils::{colour::Colour, hittable::HitRecord, material::Material, ray::Ray},
    math::vec3_ops::{dot, random_unit_vector, reflect, unit_vector},
};

pub struct Metal {
    albedo: Colour,
    fuzz: f64,
}

impl Metal {
    pub fn from(albedo: &Colour, fuzz: f64) -> Self {
        Self {
            albedo: *albedo,
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = reflect(ray_in.direction(), &rec.normal);
        reflected = unit_vector(&reflected) + (random_unit_vector() * self.fuzz);
        *scattered = Ray::from(&rec.p, &reflected);
        *attenuation = self.albedo;
        dot(scattered.direction(), &rec.normal) > 0.0
    }
}
