use crate::math::{interval::Interval, vec3::Vec3};

pub type Colour = Vec3;

pub fn write_colour(colour: &Colour, buf: &mut [u8]) {
    let intensity = Interval::from(0.0, 0.999);

    let r = linear_to_gamma(colour.x());
    let g = linear_to_gamma(colour.y());
    let b = linear_to_gamma(colour.z());

    let r = (256.0 * intensity.clamp(r)).floor() as u8;
    let g = (256.0 * intensity.clamp(g)).floor() as u8;
    let b = (256.0 * intensity.clamp(b)).floor() as u8;

    buf[0] = r;
    buf[1] = g;
    buf[2] = b;
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}
