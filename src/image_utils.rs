pub mod colour;
pub mod ray;

pub fn calculate_viewport(aspect_ratio: f64, image_width: u32) -> (f64, f64) {
    let image_height = if image_width as f64 > aspect_ratio {
        (image_width as f64 / aspect_ratio).floor() as u32
    } else {
        1
    };

    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    (viewport_height, viewport_width)
}
