use std::rc::Rc;

use simple_logger::SimpleLogger;

use ray_tracer::{
    image_utils::{
        camera::Camera,
        colour::Colour,
        hittable::Hittable,
        hittable_list::HittableList,
        material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    },
    math::vec3::Point3,
    shapes::sphere::Sphere,
};

fn main() {
    SimpleLogger::new().init().unwrap();

    log::info!("Starting up");

    let world = create_world();

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 3840;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

cam.render(&world);
}

fn create_world() -> HittableList {
    let mut world = HittableList::new();
    
    let material_ground = Rc::new(Lambertian::from(&Colour::from(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::from(&Colour::from(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::from(1.5));
    let material_bubble = Rc::new(Dielectric::from(1.0 / 1.50));
    let material_right = Rc::new(Metal::from(&Colour::from(0.8, 0.6, 0.2), 1.0));
    
    let sphere = Sphere::from(
        &Point3::from(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    );
    let sphere: Rc<dyn Hittable> = Rc::new(sphere);
    world.add(&sphere);
    let sphere = Sphere::from(&Point3::from(0.0, 0.0, -1.2), 0.5, material_center.clone());
    let sphere: Rc<dyn Hittable> = Rc::new(sphere);
    world.add(&sphere);
    let sphere = Sphere::from(&Point3::from(-1.0, 0.0, -1.0), 0.5, material_left.clone());
    let sphere: Rc<dyn Hittable> = Rc::new(sphere);
    world.add(&sphere);
    let sphere = Sphere::from(&Point3::from(-1.0, 0.0, -1.0), 0.4, material_bubble.clone());
    let sphere: Rc<dyn Hittable> = Rc::new(sphere);
    world.add(&sphere);
    let sphere = Sphere::from(&Point3::from(1.0, 0.0, -1.0), 0.5, material_right.clone());
    let sphere: Rc<dyn Hittable> = Rc::new(sphere);
    world.add(&sphere);
    world
}
