use std::{
    fs::File,
    io::{BufWriter, Write},
    sync::{Arc, Mutex},
};

use crate::{
    image_utils::{
        camera::camera_handler::CameraHandler,
        colour::{Colour, write_colour},
        hittable::{HitRecord, Hittable},
        ray::Ray,
    },
    math::{
        interval::Interval,
        utils::{INFINITY, degrees_to_radians, random_f64},
        vec3::{Point3, Vec3},
        vec3_ops::{cross, unit_vector},
    },
    thread_pool::ThreadPool,
};

mod camera_handler;

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

    pub fn render(&mut self, world: Arc<impl Hittable + 'static>) {
        self.initialize();

        let buffer = vec![0; (self.image_height as usize * self.image_width as usize + 3) * 3];
        let a_buf = Arc::new(Mutex::new(buffer));
        self.calculate_pixels(world, a_buf.clone());

        self.write_to_file(a_buf);
    }

    fn calculate_pixels(&self, world: Arc<impl Hittable + 'static>, buffer: Arc<Mutex<Vec<u8>>>) {
        let threads = ThreadPool::new(16);
        let camera_handler = Arc::new(CameraHandler::from_camera(self));
        for j in 0..self.image_height {
            let b_clone = buffer.clone();
            let world = world.clone();
            let camera_handler = camera_handler.clone();
            threads.execute(move || {
                Self::calculate_row(b_clone, world, camera_handler, j);
            });
        }
    }

    fn calculate_row(
        buffer: Arc<Mutex<Vec<u8>>>,
        world: Arc<impl Hittable>,
        a_self: Arc<CameraHandler>,
        j: u32,
    ) {
        log::debug!(
            "Scanlines remaining: {}/{}",
            a_self.image_height - j,
            a_self.image_height
        );
        for i in 0..a_self.image_width {
            let mut pixel_colour = Colour::default();
            for _ in 0..a_self.samples_per_pixel {
                let ray = a_self.get_ray(i, j);
                pixel_colour += ray_color(&ray, a_self.max_depth, world.as_ref());
            }

            let b_h = j * a_self.image_width * 3;
            let b_slice = b_h as usize + i as usize * 3;
            let b_slice = &mut buffer.lock().unwrap()[b_slice..b_slice + 3 * 3];

            write_colour(&(pixel_colour * a_self.pixel_sample_scale), b_slice);
        }
    }

    fn write_to_file(&mut self, buffer: Arc<Mutex<Vec<u8>>>) {
        let file_name = "test_a.ppm";
        log::info!("Creating {file_name}");
        let file = File::create(file_name).expect("Could not write file");
        let mut file = BufWriter::new(file);
        file.write_all(format!("P6\n{} {}\n255\n", self.image_width, self.image_height).as_bytes())
            .expect("Could not write header");
        file.write_all(buffer.lock().unwrap().as_slice())
            .expect("Could not write buffer to file");
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
