use crate::file_utils::create_and_write_to_file;

pub fn create_test_image() {
    let w = 256;
    let h = 256;

    let mut content = format!("P3\n{w} {h}\n255\n");
    for j in 0..h {
        log::debug!("Scanlines remaining: {}", h - j);
        for i in 0..w {
            let r = i as f64 / (w - 1) as f64;
            let g = j as f64 / (h - 1) as f64;
            // let b = ((i+j) as f64/2 as f64) / ((h+w)/2 - 1) as f64;
            let b = 0;

            let r = (r * 255.99).floor() as u8;
            let g = (g * 255.99).floor() as u8;
            // let b = (b * 255.99).floor() as u8;

            content += format!("{r} {g} {b} ").as_str();
        }
        content += "\n";
    }

    create_and_write_to_file("test_a.ppm", content.as_str()).expect("Could not create file");
    log::info!("Done!");
}
