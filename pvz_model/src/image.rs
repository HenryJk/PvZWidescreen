#![allow(non_snake_case)]

use std::ffi::c_void;

use windows::Win32::Graphics::DirectDraw::{IDirectDrawSurface, DDSURFACEDESC};

use crate::{
    stub::{StdBasicString, StdVector},
    AnimType, DDInterface, SexyAppBase,
};

#[repr(C)]
pub struct AnimInfo {
    pub mAnimType: AnimType,
    pub mFrameDelay: i32,
    pub mNumCels: i32,
    pub mPerFrameDelay: StdVector,
    pub mFrameMap: StdVector,
    pub mTotalAnimTime: i32,
}

#[repr(C)]
pub struct Image {
    unknown: [u8; 4],
    pub mDrawn: bool,
    pub mFilePath: StdBasicString,
    pub mWidth: i32,
    pub mHeight: i32,
    pub mNumRows: i32,
    pub mNumCols: i32,
    pub mAnimInfo: *mut AnimInfo,
}

#[repr(C)]
pub struct MemoryImage {
    pub base: Image,
    pub mBits: *mut u32,
    pub mBitsChangedCount: i32,
    pub mD3DData: *mut c_void,
    pub mD3DFlags: u32,
    pub mColorTable: *mut u32,
    pub mColorIndices: *mut u8,
    pub mForcedMode: bool,
    pub mHasTrans: bool,
    pub mHasAlpha: bool,
    pub mIsVolatile: bool,
    pub mPurgeBits: bool,
    pub mWantPal: bool,
    pub mNativeAlphaData: *mut u32,
    pub mRLAlphaData: *mut u8,
    pub mRLAdditiveData: *mut u8,
    pub mBitsChanged: bool,
    pub mApp: *mut SexyAppBase,
}

#[repr(C)]
pub struct DDImage {
    pub base: MemoryImage,
    pub mDDInterface: *mut DDInterface,
    pub mSurface: *mut IDirectDrawSurface,
    pub mSurfaceSet: bool,
    pub mNoLock: bool,
    pub mVideoMemory: bool,
    pub mFirstPixelTrans: bool,
    pub mWantDDSurface: bool,
    pub mDrawToBits: bool,
    pub mLockCount: i32,
    pub mLockedSurfaceDesc: DDSURFACEDESC,
}
