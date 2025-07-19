use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::{
    image_utils::{
        colour::{Colour, write_colour},
        hittable::{HitRecord, Hittable},
        ray::Ray,
    },
    math::{
        interval::Interval,
        utils::{INFINITY, degrees_to_radians, random_f64},
        vec3::{Point3, Vec3},
        vec3_ops::{cross, random_in_unit_disk, unit_vector},
    },
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,

    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,

    pub defocus_angle: f64,
    pub focus_dist: f64,

    image_height: u32,
    pixel_sample_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            defocus_angle: 0.0,
            focus_dist: 10.0,
            lookfrom: Point3::default(),
            lookat: Point3::from(0.0, 0.0, -1.0),
            vup: Vec3::from(0.0, 1.0, 0.0),
            image_height: 0,
            pixel_sample_scale: 1.0,
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
        }
    }

    pub fn render(&mut self, world: &impl Hittable) {
        self.initialize();

        let file_name = "test_a.ppm";
        log::info!("Creating {file_name}");
        let file = File::create(file_name).expect("Could not write file");
        let mut file = BufWriter::new(file);
        file.write_all(format!("P6\n{} {}\n255\n", self.image_width, self.image_height).as_bytes())
            .expect("Could not write header");
        let mut buffer = vec![0; (self.image_height as usize * self.image_width as usize + 3) * 3];

        for j in 0..self.image_height {
            log::debug!(
                "Scanlines remaining: {}/{}",
                self.image_height - j,
                self.image_height
            );
            let b_h = j * self.image_width * 3;
            for i in 0..self.image_width {
                let mut pixel_colour = Colour::default();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_colour += ray_color(&ray, self.max_depth, world);
                }

                let b_slice = b_h as usize + i as usize * 3;
                let b_slice = &mut buffer[b_slice..b_slice + 3 * 3];
                write_colour(&(pixel_colour * self.pixel_sample_scale), b_slice);
            }
        }

        file.write_all(buffer.as_slice())
            .expect("Could not write buffer to file");

        log::info!("Done!");
    }

    fn initialize(&mut self) {
        self.image_height = if self.image_width as f64 > self.aspect_ratio {
            (self.image_width as f64 / self.aspect_ratio) as u32
        } else {
            1
        };

        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.lookfrom;

        let theta = degrees_to_radians(self.vfov);
        let h = f64::tan(theta / 2.0);

        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = unit_vector(&(self.lookfrom - self.lookat));
        self.u = unit_vector(&cross(&self.vup, &self.w));
        self.v = cross(&self.w, &self.u);

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * self.v.negate();

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;

        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius =
            self.focus_dist * f64::tan(degrees_to_radians(self.defocus_angle / 2.0));
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    /// Construct a camera ray originating from the defocus disk and derected at a randomly
    /// sampled point around the pixel location i,j.
    fn get_ray(&self, i: u32, j: u32) -> Ray {
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

fn ray_color(ray: &Ray, depth: u32, world: &impl Hittable) -> Colour {
    if depth == 0 {
        return Colour::default();
    }

    let mut rec = HitRecord::default();

    if world.hit(ray, &Interval::from(0.001, INFINITY), &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Colour::default();
        if rec.mat.scatter(ray, &rec, &mut attenuation, &mut scattered) {
            return ray_color(&scattered, depth - 1, world) * attenuation;
        }
        return Colour::default();
    }

    let unit_direction = unit_vector(ray.direction());
    let a = (unit_direction.y() + 1.0) * 0.5;
    blended_value(a, Colour::from(1.0, 1.0, 1.0), Colour::from(0.5, 0.7, 1.0))
}

pub fn blended_value(a: f64, start_value: Colour, end_value: Colour) -> Colour {
    start_value * (1.0 - a) + end_value * a
}

fn sample_square() -> Vec3 {
    Vec3::from(random_f64() - 0.5, random_f64() - 0.5, 0.0)
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
