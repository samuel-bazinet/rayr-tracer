use std::{sync::Arc, time::SystemTime};

use simple_logger::SimpleLogger;

use ray_tracer::{
    image_utils::{
        camera::Camera,
        colour::Colour,
        hittable::Hittable,
        hittable_list::HittableList,
        material::{Material, dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    },
    math::{
        utils::{PI, random_f64, random_f64_bounded},
        vec3::{Point3, Vec3},
    },
    shapes::sphere::Sphere,
};

fn main() {
    SimpleLogger::new().init().unwrap();

    log::info!("Starting up");
    let now = SystemTime::now();

    let world = _create_book_cover();

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 3840;
    cam.samples_per_pixel = 100;
    cam.max_depth = 10;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::from(13.0, 2.0, 3.0);
    cam.lookat = Point3::from(0.0, 0.0, 0.0);
    cam.vup = Vec3::from(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6; // 0.6 to match the book, 0 to remove blur
    cam.focus_dist = 10.0;

    cam.render(Arc::new(world));

    log::info!("Done in {} seconds!", now.elapsed().unwrap().as_secs());
}

fn _create_world_1() -> HittableList {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::from(&Colour::from(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::from(&Colour::from(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::from(1.5));
    let material_bubble = Arc::new(Dielectric::from(1.0 / 1.50));
    let material_right = Arc::new(Metal::from(&Colour::from(0.8, 0.6, 0.2), 1.0));

    let sphere = Sphere::from(
        &Point3::from(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    );
    let sphere: Arc<dyn Hittable> = Arc::new(sphere);
    world.add(&sphere);
    let sphere = Sphere::from(&Point3::from(0.0, 0.0, -1.2), 0.5, material_center.clone());
    let sphere: Arc<dyn Hittable> = Arc::new(sphere);
    world.add(&sphere);
    let sphere = Sphere::from(&Point3::from(-1.0, 0.0, -1.0), 0.5, material_left.clone());
    let sphere: Arc<dyn Hittable> = Arc::new(sphere);
    world.add(&sphere);
    let sphere = Sphere::from(&Point3::from(-1.0, 0.0, -1.0), 0.4, material_bubble.clone());
    let sphere: Arc<dyn Hittable> = Arc::new(sphere);
    world.add(&sphere);
    let sphere = Sphere::from(&Point3::from(1.0, 0.0, -1.0), 0.5, material_right.clone());
    let sphere: Arc<dyn Hittable> = Arc::new(sphere);
    world.add(&sphere);
    world
}

fn _create_world_2() -> HittableList {
    let mut world = HittableList::new();

    let r = f64::cos(PI / 4.0);

    let material_left = Arc::new(Lambertian::from(&Colour::from(0.0, 0.0, 1.0)));
    let material_right = Arc::new(Lambertian::from(&Colour::from(1.0, 0.0, 0.0)));

    let sphere = Arc::new(Sphere::from(
        &Point3::from(-r, 0.0, -1.0),
        r,
        material_left.clone(),
    )) as Arc<dyn Hittable>;
    world.add(&sphere);
    let sphere = Arc::new(Sphere::from(
        &Point3::from(r, 0.0, -1.0),
        r,
        material_right.clone(),
    )) as Arc<dyn Hittable>;
    world.add(&sphere);

    world
}

fn _create_book_cover() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::from(&Colour::from(0.5, 0.5, 0.5)));
    world.add(
        &(Arc::new(Sphere::from(
            &Point3::from(0.0, -1000.0, 0.0),
            1000.0,
            ground_material.clone(),
        )) as Arc<dyn Hittable>),
    );

    let glass_material = Arc::new(Dielectric::from(1.5));

    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::from(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if (center - Point3::from(4.0, 0.2, 0.0)).length() > 0.9 {
                let choose_mat = random_f64();
                let sphere_material: Arc<dyn Material>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Colour::random() * Colour::random();
                    sphere_material = Arc::new(Lambertian::from(&albedo));
                } else if choose_mat < 0.95 {
                    let albedo = Colour::random_bounded(0.5, 1.0);
                    let fuzz = random_f64_bounded(0.0, 0.5);
                    sphere_material = Arc::new(Metal::from(&albedo, fuzz));
                } else {
                    sphere_material = glass_material.clone();
                }

                world.add(
                    &(Arc::new(Sphere::from(&center, 0.2, sphere_material.clone()))
                        as Arc<dyn Hittable>),
                );
            }
        }
    }

    let material1 = Arc::new(Dielectric::from(1.5));
    world.add(
        &(Arc::new(Sphere::from(
            &Point3::from(0.0, 1.0, 0.0),
            1.0,
            material1.clone(),
        )) as Arc<dyn Hittable>),
    );

    let material2 = Arc::new(Lambertian::from(&Colour::from(0.4, 0.2, 0.1)));
    world.add(
        &(Arc::new(Sphere::from(
            &Point3::from(-4.0, 1.0, 0.0),
            1.0,
            material2.clone(),
        )) as Arc<dyn Hittable>),
    );

    let material3 = Arc::new(Metal::from(&Colour::from(0.7, 0.6, 0.5), 0.0));
    world.add(
        &(Arc::new(Sphere::from(
            &Point3::from(4.0, 1.0, 0.0),
            1.0,
            material3.clone(),
        )) as Arc<dyn Hittable>),
    );

    world
}
