#![allow(non_snake_case)]

use crate::{Color, DataArray, FloatParameterTrack, Image, SexyVector2};

#[repr(C)]
pub struct TrailDefinition {
    pub mImage: *mut Image,
    pub mMaxPoints: i32,
    pub mMinPointDistance: f32,
    pub mTrailFlags: i32,
    pub mTrailDuration: FloatParameterTrack,
    pub mWidthOverLength: FloatParameterTrack,
    pub mWidthOverTime: FloatParameterTrack,
    pub mAlphaOverLength: FloatParameterTrack,
    pub mAlphaOverTime: FloatParameterTrack,
}

#[repr(C)]
pub struct Trail {
    pub mTrailPoints: [SexyVector2; 20],
    pub mNumTrailPoints: i32,
    pub mDead: bool,
    pub mRenderOrder: i32,
    pub mTrailAge: i32,
    pub mTrailDuration: i32,
    pub mDefinition: *mut TrailDefinition,
    pub mTrailHolder: *mut TrailHolder,
    pub mTrailInterp: [f32; 4],
    pub mTrailCenter: SexyVector2,
    pub mIsAttachment: bool,
    pub mColorOverride: Color,
}

#[repr(C)]
pub struct TrailHolder {
    pub mTrails: DataArray<Trail>,
}
