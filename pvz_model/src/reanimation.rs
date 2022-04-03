#![allow(non_snake_case)]

use crate::{
    Color, DataArray, FilterEffect, Font, Image, MemoryImage, ReanimLoopType, ReanimationType,
    SexyTransform2D,
};

#[repr(C)]
pub struct ReanimatorTransform {
    pub mTransX: f32,
    pub mTransY: f32,
    pub mSkewX: f32,
    pub mSkewY: f32,
    pub mScaleX: f32,
    pub mScaleY: f32,
    pub mFrame: f32,
    pub mAlpha: f32,
    pub mImage: *mut Image,
    pub mFont: *mut Font,
    pub mText: *mut char,
}

#[repr(C)]
pub struct ReanimAtlasImage {
    pub mX: i32,
    pub mY: i32,
    pub mWidth: i32,
    pub mHeight: i32,
    pub mOriginalImage: *mut Image,
}

#[repr(C)]
pub struct ReanimatorTrack {
    pub mName: *mut char,
    pub mTransforms: *mut ReanimatorTransform,
    pub mTransformCount: i32,
}

#[repr(C)]
pub struct ReanimAtlas {
    pub mImageArray: [ReanimAtlasImage; 64],
    pub mImageCount: i32,
    pub mMemoryImage: *mut MemoryImage,
}

#[repr(C)]
pub struct ReanimatorDefinition {
    pub mTracks: *mut ReanimatorTrack,
    pub mTrackCount: i32,
    pub mFPS: f32,
    pub mReanimAtlas: *mut ReanimAtlas,
}

#[repr(C)]
pub struct ReanimatorTrackInstance {
    pub mBlendCounter: i32,
    pub mBlendTime: i32,
    pub mBlendTransform: ReanimatorTransform,
    pub mShakeOverride: f32,
    pub mShakeX: f32,
    pub mShakeY: f32,
    pub mAttachmentID: i32,
    pub mImageOverride: *mut Image,
    pub mRenderGroup: i32,
    pub mTrackColor: Color,
    pub mIgnoreClipRect: bool,
    pub mTruncateDisappearingFrames: bool,
}

#[repr(C)]
pub struct Reanimation {
    pub mReanimationType: ReanimationType,
    pub mAnimTime: f32,
    pub mAnimRate: f32,
    pub mDefinition: *mut ReanimatorDefinition,
    pub mLoopType: ReanimLoopType,
    pub mDead: bool,
    pub mFrameStart: i32,
    pub mFrameCount: i32,
    pub mFrameBasePose: i32,
    pub mOverlayMatrix: SexyTransform2D,
    pub mColorOverride: Color,
    pub mTrackInstances: *mut ReanimatorTrackInstance,
    pub mLoopCount: i32,
    pub mReanimationHolder: *mut ReanimationHolder,
    pub mIsAttachment: bool,
    pub mRenderOrder: i32,
    pub mExtraAdditiveColor: Color,
    pub mEnableExtraAdditiveDraw: bool,
    pub mExtraOverlayColor: Color,
    pub mEnableExtraOverlayDraw: bool,
    pub mLastFrameTime: f32,
    pub mFilterEffect: FilterEffect,
}

#[repr(C)]
pub struct ReanimationHolder {
    pub mReanimations: DataArray<Reanimation>,
}
