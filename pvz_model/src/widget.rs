#![allow(non_snake_case)]

use std::ffi::c_void;

use crate::{
    stub::{StdList, StdVector},
    Graphics, MemoryImage, SexyAppBase, TRect,
};

#[repr(C)]
pub struct FlagsMod {
    pub mAddFlags: i32,
    pub mRemoveflags: i32,
}

#[repr(C)]
pub struct Insets {
    pub mLeft: i32,
    pub mTop: i32,
    pub mRight: i32,
    pub mBottom: i32,
}

#[repr(C)]
pub struct WidgetContainer {
    unknown: [u8; 4],
    pub mWidgets: StdList,
    pub mWidgetManager: *const c_void,
    pub mParent: *const WidgetContainer,
    pub mUpdateIteratorModified: bool,
    pub mUpdateIterator: [u8; 8],
    pub mLastWMUpdateCount: u32,
    pub mUpdateCnt: i32,
    pub mDirty: bool,
    pub mX: i32,
    pub mY: i32,
    pub mWidth: i32,
    pub mHeight: i32,
    pub hasAlpha: bool,
    pub mClip: bool,
    pub mWidgetFlagsMod: FlagsMod,
    pub mPriority: i32,
    pub mZOrder: i32,
}

#[repr(C)]
pub struct Widget {
    pub base: WidgetContainer,
    pub mVisible: bool,
    pub mMouseVisible: bool,
    pub mDisabled: bool,
    pub mHasFocus: bool,
    pub mIsDown: bool,
    pub mIsOver: bool,
    pub mHasTransparencies: bool,
    pub mColors: StdVector,
    pub mMouseInsets: Insets,
    pub mDoFinger: bool,
    pub mWantsFocus: bool,
    pub mTabPrev: *const Widget,
    pub mTabNext: *const Widget,
}

#[repr(C)]
pub struct WidgetManager {
    pub mDefaultTab: *mut Widget,
    pub mCurG: *mut Graphics,
    pub mApp: *mut SexyAppBase,
    pub mImage: *mut MemoryImage,
    pub mTransientImage: *mut MemoryImage,
    pub mLastHadTransients: bool,
    pub mPopupCommandWidget: *mut Widget,
    pub mDeferredOverlayWidgets: StdVector,
    pub mMinDeferredOverlayPriority: i32,
    pub mHasFocus: bool,
    pub mFocusWidget: *mut Widget,
    pub mLastDownWidget: *mut Widget,
    pub mOverWidget: *mut Widget,
    pub mBaseModalWidget: *mut Widget,
    pub mLostFocusFlagsMod: FlagsMod,
    pub mBelowModalFlagsMod: FlagsMod,
    pub mDefaultBelowModalFlagsMod: FlagsMod,
    pub mPreModalInfoList: FlagsMod,
    pub mMouseDestRect: TRect<i32>,
    pub mMouseSourceRect: TRect<i32>,
    pub mMouseIn: bool,
    pub mLastMouseX: i32,
    pub mLastMouseY: i32,
    pub mDownButtons: i32,
    pub mActualDownButtons: i32,
    pub mLastInputUpdateCnt: i32,
    pub mKeyDown: [bool; 255],
    pub mLastDownButtonId: i32,
    pub mWidgetFlags: i32,
}
