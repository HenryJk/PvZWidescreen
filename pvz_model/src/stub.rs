#![allow(non_snake_case)]

#[repr(C)]
pub struct IDirect3D7 {
    unknown: [u8; 4],
}

#[repr(C)]
pub struct IDirect3DDevice7 {
    unknown: [u8; 4],
}

#[repr(C)]
pub struct StdBasicString {
    unknown: [u8; 28],
}

#[repr(C)]
pub struct StdVector {
    unknown: [u8; 16],
}

#[repr(C)]
pub struct StdMap {
    unknown: [u8; 12],
}

#[repr(C)]
pub struct StdList {
    unknown: [u8; 12],
}

#[repr(C)]
pub struct StdSet {
    unknown: [u8; 12],
}

#[repr(C)]
pub struct ResourceManager {
    unknown: [u8; 204],
}
