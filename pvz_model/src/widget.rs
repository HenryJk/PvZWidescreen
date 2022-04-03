#![allow(non_snake_case)]

use crate::{
    stub::{StdBasicString, StdList, StdVector},
    ButtonListener, Color, Font, Graphics, Image, MemoryImage, SexyAppBase, TRect, DialogButton,
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
    pub mWidgetManager: *mut WidgetManager,
    pub mParent: *mut WidgetContainer,
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
pub struct ButtonWidget {
    pub base: Widget,
    pub mId: i32,
    pub mLabel: StdBasicString,
    pub mLabelJustify: i32,
    pub mFont: *mut Font,
    pub mButtonImage: *mut Image,
    pub mOverImage: *mut Image,
    pub mDownImage: *mut Image,
    pub mDisabledImage: *mut Image,
    pub mNormalRect: TRect<i32>,
    pub mOverRect: TRect<i32>,
    pub mDownRect: TRect<i32>,
    pub mDisabledRect: TRect<i32>,
    pub mInverted: bool,
    pub mBtnNoDraw: bool,
    pub mFrameNoDraw: bool,
    pub mButtonListener: *mut ButtonListener,
    pub mOverAlpha: f64,
    pub mOverAlphaSpeed: f64,
    pub mOverAlphaFadeInSpeed: f64,
}


#[repr(C)]
pub struct NewLawnButton {
    pub base: DialogButton,
    pub mHiliteFont: *mut Font,
    pub mTextDownOffsetX: i32,
    pub mTextDownOffsetY: i32,
    unknown: u32,
}

#[repr(C)]
pub struct HyperlinkWidget {
    pub base: ButtonWidget,
    pub mColor: Color,
    pub mOverColor: Color,
    pub mUnderlineSize: i32,
    pub mUnderlineOffset: i32,
}

#[repr(C)]
pub struct WidgetManager {
    pub base: WidgetContainer,
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
    pub mPreModalInfoList: StdList,
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

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::{ButtonWidget, FlagsMod, Insets, Widget, WidgetContainer, WidgetManager};

    #[test]
    fn check_FlagsMod_size() {
        assert_eq!(size_of::<FlagsMod>(), 8);
    }

    #[test]
    fn check_Insets_size() {
        assert_eq!(size_of::<Insets>(), 16);
    }

    #[test]
    fn check_WidgetContainer_size() {
        assert_eq!(size_of::<WidgetContainer>(), 84);
    }

    #[test]
    fn check_Widget_size() {
        assert_eq!(size_of::<Widget>(), 136);
    }

    #[test]
    fn check_ButtonWidget_size() {
        assert_eq!(size_of::<ButtonWidget>(), 288);
    }

    #[test]
    fn check_WidgetManager_size() {
        assert_eq!(size_of::<WidgetManager>(), 508);
    }
}
