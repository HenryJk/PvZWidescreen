#![allow(non_snake_case)]

use core::ffi::c_void;

#[repr(C)]
pub struct IDirect3D7 {
    unknown: *const c_void,
}

#[repr(C)]
pub struct StdBasicString {
    unknown: [u32; 7],
}

#[repr(C)]
pub struct StdVector {
    unknown: [u32; 4],
}

#[repr(C)]
pub struct StdMap {
    unknown: [u32; 3],
}

#[repr(C)]
pub struct StdList {
    unknown: [u32; 3],
}

#[repr(C)]
pub struct StdSet {
    unknown: [u32; 3],
}

#[repr(C)]
pub struct InternetManager {}

#[repr(C)]
pub struct BetaSupport {
    unknown: [u8; 592],
}

#[repr(C)]
pub struct ReanimatorCache {}

#[repr(C)]
pub struct ProfileMgr {}

#[repr(C)]
pub struct PlayerInfo {}

#[repr(C)]
pub struct LevelStats {}

#[repr(C)]
pub struct PopDRMComm {}

#[repr(C)]
pub struct Music {}

#[repr(C)]
pub struct D3DDEVICEDESC7 {}
