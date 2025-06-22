use crate::file_utils::create_and_write_to_file;
use crate::image_utils::{write_colour, Colour};

pub fn create_test_image() {
    let w = 256;
    let h = 256;

    let mut content = format!("P3\n{w} {h}\n255\n");
    for j in 0..h {
        log::debug!("Scanlines remaining: {}", h - j);
        for i in 0..w {
           let colour = Colour::from_vals(i as f64/(w-1) as f64, j as f64/(h-1) as f64, 0.0);

            content += write_colour(&colour).as_str();
        }
        content += "\n";
    }

    create_and_write_to_file("test_a.ppm", content.as_str()).expect("Could not create file");
    log::info!("Done!");
}
