#![allow(non_snake_case)]

use crate::{HyperlinkWidget, KeyCode, LawnApp, Widget};

#[repr(C)]
pub struct TitleScreen {
    pub base: Widget,
    unknown: [u8; 4],
    pub mStartButton: *mut HyperlinkWidget,
    pub mCurBarWidth: f32,
    pub mTotalBarWidth: f32,
    pub mBarVel: f32,
    pub mRegisterClicked: bool,
    pub mLoadingThreadComplete: bool,
    pub mTitleAge: i32,
    pub mQuickLoadKey: KeyCode,
    pub mNeedRegister: bool,
    pub mNeedShowRegisterBox: bool,
    pub mDrawnYet: bool,
    pub mNeedToInit: bool,
    pub mPrevLoadingPercent: f32,
    pub mApp: *mut LawnApp,
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::TitleScreen;

    #[test]
    fn check_TitleScreen_size() {
        assert_eq!(size_of::<TitleScreen>(), 180);
    }
}
