#![allow(non_snake_case)]

#[repr(C)]
pub struct SexyMatrix3 {
    pub m00: f32,
    pub m01: f32,
    pub m02: f32,
    pub m10: f32,
    pub m11: f32,
    pub m12: f32,
    pub m20: f32,
    pub m21: f32,
    pub m22: f32,
}

#[repr(C)]
pub struct SexyTransform2D {
    pub base: SexyMatrix3,
}
