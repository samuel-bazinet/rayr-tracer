use crate::math::vec3::Vec3;

pub type Colour = Vec3;

pub fn write_colour(colour: &Colour) -> String {
    let r = (colour.x() * 255.99).floor() as u8;
    let g = (colour.y() * 255.99).floor() as u8;
    let b = (colour.z() * 255.99).floor() as u8;

    format!("{r} {g} {b}\n")
}
