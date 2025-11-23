use crate::{
    image_utils::{
        camera::{Camera, sample_square},
        ray::Ray,
    },
    math::{
        vec3::{Point3, Vec3},
        vec3_ops::random_in_unit_disk,
    },
};

pub struct CameraHandler {
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    defocus_angle: f64,
    pub image_height: u32,
    pub image_width: u32,
    pub pixel_sample_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl CameraHandler {
    pub fn from_camera(camera: &Camera) -> Self {
        Self {
            samples_per_pixel: camera.samples_per_pixel,
            max_depth: camera.max_depth,
            defocus_angle: camera.defocus_angle,
            image_height: camera.image_height,
            image_width: camera.image_width,
            pixel_sample_scale: camera.pixel_sample_scale,
            center: camera.center,
            pixel00_loc: camera.pixel00_loc,
            pixel_delta_u: camera.pixel_delta_u,
            pixel_delta_v: camera.pixel_delta_v,
            defocus_disk_u: camera.defocus_disk_u,
            defocus_disk_v: camera.defocus_disk_v,
        }
    }

    /// Construct a camera ray originating from the defocus disk and derected at a randomly
    /// sampled point around the pixel location i,j.
    pub fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (i as f64 + offset.x()))
            + (self.pixel_delta_v * (j as f64 + offset.y()));

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::from(&ray_origin, &ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.center + (p.e[0] * self.defocus_disk_u) + (p.e[1] * self.defocus_disk_v)
    }
}
