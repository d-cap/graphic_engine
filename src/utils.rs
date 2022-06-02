use std::f32::consts::PI;

#[inline]
pub fn to_radians(angle: f32) -> f32 {
    angle * (PI / 180.)
}
