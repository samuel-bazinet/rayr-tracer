use crate::file_utils::create_and_write_to_file;
use crate::image_utils::hittable_list::HittableList;
use crate::image_utils::{
    colour::write_colour,
    ray::{Ray, ray_color},
};
use crate::math::vec3::{Point3, Vec3};

pub fn create_test_image(
    w: u32,
    h: u32,
    pixel00_loc: &Vec3,
    pixel_delta_u: &Vec3,
    pixel_delta_v: &Vec3,
    camera_center: &Point3,
    world: &HittableList,
) {
    let mut content = format!("P3\n{w} {h}\n255\n");
    for j in 0..h {
        log::debug!("Scanlines remaining: {}", h - j);
        for i in 0..w {
            let pixel_center =
                *pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - *camera_center;
            let ray = Ray::from(camera_center, &ray_direction);

            let colour = ray_color(&ray, world);

            content += write_colour(&colour).as_str();
        }
        content += "\n";
    }

    create_and_write_to_file("test_a.ppm", content.as_str()).expect("Could not create file");
    log::info!("Done!");
}
