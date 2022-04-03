#![allow(non_snake_case)]

#[repr(C)]
pub struct MusicInterface {
    unknown: u32,
}

#[repr(C)]
pub struct SoundManager {
    unknown: u32,
}

#[repr(C)]
pub struct SoundInstance {
    unknown: u32,
}

#[repr(C)]
pub struct FoleyInstance {
    pub mInstance: *mut SoundInstance,
    pub mRefCount: i32,
    pub mPaused: bool,
    pub mStartTime: i32,
    pub mPauseOffset: i32,
}

#[repr(C)]
pub struct FoleyTypeData {
    pub mFoleyInstances: [FoleyInstance; 8],
    pub mLastVariationPlayed: i32,
}

#[repr(C)]
pub struct TodFoley {
    pub mFoleyTypeData: [FoleyTypeData; 110],
}
