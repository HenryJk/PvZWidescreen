#![allow(non_snake_case)]

use std::ffi::c_void;

use crate::{stub::StdList, Color, TRect};

#[repr(C)]
pub struct Edge {
    mX: f64,
    mDX: f64,
    i: i32,
    unknown: [u8; 4],
    b: f64,
}

#[repr(C)]
pub struct GraphicsState {
    pub mDestImage: *const c_void,
    pub mTransX: f32,
    pub mTransY: f32,
    pub mScaleX: f32,
    pub mScaleY: f32,
    pub mScaleOrigX: f32,
    pub mScaleOrigY: f32,
    pub mClipRect: TRect<i32>,
    pub mColor: Color,
    pub mFont: *const c_void,
    pub mDrawMode: i32,
    pub mColorizeImages: bool,
    pub mFastStretch: bool,
    pub mWriteColoredString: bool,
    pub mLinearBlend: bool,
    pub mIs3D: bool,
}

#[repr(C)]
pub struct Graphics {
    unknown: [u8; 4],
    pub state: GraphicsState,
    pub mPFActiveEdgeList: *const Edge,
    pub mPFNumActiveEdges: i32,
    pub mPFNumVertices: i32,
    pub mStateStack: StdList,
}
