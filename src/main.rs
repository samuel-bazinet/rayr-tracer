use std::rc::Rc;

use simple_logger::SimpleLogger;

use ray_tracer::{
    image_utils::{camera::Camera, hittable::Hittable, hittable_list::HittableList},
    math::vec3::Point3,
    shapes::sphere::Sphere,
};

fn main() {
    SimpleLogger::new().init().unwrap();

    log::info!("Starting up");

    let mut world = HittableList::new();
    let sphere = Sphere::from(&Point3::from(0.0, 0.0, -1.0), 0.5);
    let sphere: Rc<dyn Hittable> = Rc::new(sphere);
    world.add(&sphere);
    let sphere = Sphere::from(&Point3::from(0.0, -100.5, -1.0), 100.0);
    let sphere: Rc<dyn Hittable> = Rc::new(sphere);
    world.add(&sphere);

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;

    cam.render(&world);
}
