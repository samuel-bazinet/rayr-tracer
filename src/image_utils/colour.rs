use crate::math::{interval::Interval, vec3::Vec3};

pub type Colour = Vec3;

pub fn write_colour(colour: &Colour) -> String {
    let intensity = Interval::from(0.0, 0.999);

    let r = colour.x();
    let g = colour.y();
    let b = colour.z();

    let r = (256.0 * intensity.clamp(r)).floor() as u8;
    let g = (256.0 * intensity.clamp(g)).floor() as u8;
    let b = (256.0 * intensity.clamp(b)).floor() as u8;

    format!("{r} {g} {b}\n")
}
