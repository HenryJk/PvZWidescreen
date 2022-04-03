#![allow(non_snake_case)]

use crate::{Image, Widget, stub::StdBasicString, Font, Insets};

#[repr(C)]
pub struct DialogListener {
    unknown: [u8; 4],
}

#[repr(C)]
pub struct DialogButton {
    pub mComponentImage: *mut Image,
    pub mTranslateX: i32,
    pub mTranslateY: i32,
    pub mTextOffsetX: i32,
    pub mTextOffsetY: i32,
    unknown: u32,
}

#[repr(C)]
pub struct Dialog {
    pub base: Widget,
    unknown: [u8; 4],
    pub mDialogListener: *mut DialogListener,
    pub mComponentImage: *mut Image,
    pub mYesButton: *mut DialogButton,
    pub mNoButton: *mut DialogButton,
    pub mNumButtons: i32,
    pub mDialogHeader: StdBasicString,
    pub mDialogFooter: StdBasicString,
    pub mDialogLines: StdBasicString,
    pub mButtonMode: i32,
    pub mHeaderFont: *mut Font,
    pub mLinesFont: *mut Font,
    pub mTextAlign: i32,
    pub mLineSpacingOffset: i32,
    pub mButtonHeight: i32,
    pub mBackgroundInsets: Insets,
    pub mContentInsets: Insets,
    pub mSpaceAfterHeader: i32,
    pub mDragging: bool,
    pub mDragMouseX: i32,
    pub mDragMouseY: i32,
    pub mId: i32,
    pub mIsModal: bool,
    pub mResult: i32,
    pub mButtonHorzSpacing: i32,
    pub mButtonSidePadding: i32,
}
