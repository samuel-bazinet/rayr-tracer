use std::rc::Rc;

use simple_logger::SimpleLogger;

use ray_tracer::{
    image_utils::{calculate_viewport, hittable::Hittable, hittable_list::HittableList},
    math::vec3::{Point3, Vec3},
    shapes::sphere::Sphere,
    test_lib::create_test_image,
};

fn main() {
    SimpleLogger::new().init().unwrap();

    log::info!("Starting up");

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = if image_width as f64 > aspect_ratio {
        (image_width as f64 / aspect_ratio).floor() as u32
    } else {
        1
    };

    let mut world = HittableList::new();
    let sphere = Sphere::from(&Point3::from(0.0, 0.0, -1.0), 0.5);
    let sphere: Rc<dyn Hittable> = Rc::new(sphere);
    world.add(&sphere);
    let sphere = Sphere::from(&Point3::from(0.0, -100.5, -1.0), 100.0);
    let sphere: Rc<dyn Hittable> = Rc::new(sphere);
    world.add(&sphere);

    let focal_length = 1.0;

    let (viewport_height, viewport_width) = calculate_viewport(aspect_ratio, image_width);

    let camera_center = Point3::default();

    let viewport_u = Vec3::from(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::from(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_center - Vec3::from(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    create_test_image(
        image_width,
        image_height,
        &pixel00_loc,
        &pixel_delta_u,
        &pixel_delta_v,
        &camera_center,
        &world,
    );
}
