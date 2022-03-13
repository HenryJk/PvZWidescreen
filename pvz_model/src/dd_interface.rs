#![allow(non_snake_case)]

use crate::{
    stub::{IDirect3D7, StdBasicString, StdList, StdSet},
    DDImage, IDirect3DDevice7, Image, Ratio, SexyAppBase, TRect, _D3DVIEWPORT7,
};

use windows::Win32::{
    Foundation::HWND,
    Graphics::DirectDraw::{IDirectDraw, IDirectDraw7, IDirectDrawSurface, IDirectDrawSurface7},
    System::Threading::RTL_CRITICAL_SECTION,
};

#[repr(C)]
pub struct D3DInterface {
    unknown: [u8; 4],
    pub mHWnd: *mut HWND,
    pub mWidth: i32,
    pub mHeight: i32,
    pub mDD: *mut IDirectDraw7,
    pub mDDSDrawSurface: *mut IDirectDrawSurface7,
    pub mZBuffer: *mut IDirectDrawSurface7,
    pub mD3D: *mut IDirect3D7,
    pub mD3DDevice: *mut IDirect3DDevice7,
    pub mD3DViewport: _D3DVIEWPORT7,
    pub mSceneBegun: bool,
    pub mIsWindowed: bool,
    pub mImageSet: StdSet,
    pub mTransformStack: StdList,
}

#[repr(C)]
pub struct D3DTester {}

#[repr(C)]
pub struct NativeDisplay {
    unknown: [u8; 4],
    pub mRGBBits: i32,
    pub mRedMask: u32,
    pub mGreenMask: u32,
    pub mBlueMask: u32,
    pub mRedBits: i32,
    pub mGreenBits: i32,
    pub mBlueBits: i32,
    pub mRedShift: i32,
    pub mGreenShift: i32,
    pub mBlueShift: i32,
}

#[repr(C)]
pub struct DDInterface {
    pub base: NativeDisplay,
    pub mApp: *mut SexyAppBase,
    pub mD3DInterface: *mut D3DInterface,
    pub mD3DTester: *mut D3DTester,
    pub mIs3D: bool,
    pub mCritSect: RTL_CRITICAL_SECTION,
    pub mInRedraw: bool,
    pub mDD: *mut IDirectDraw,
    pub mDD7: *mut IDirectDraw7,
    pub mPrimarySurface: *mut IDirectDrawSurface,
    pub mSecondarySurface: *mut IDirectDrawSurface,
    pub mDrawSurface: *mut IDirectDrawSurface,
    pub mWindowScaleBuffers: [u8; 16],
    pub mWidth: i32,
    pub mHeight: i32,
    pub mAspect: Ratio,
    pub mDesktopWidth: i32,
    pub mDesktopHeight: i32,
    pub mDesktopAspect: Ratio,
    pub mIsWidescreen: bool,
    pub mDisplayWidth: i32,
    pub mDisplayHeight: i32,
    pub mDisplayAspect: Ratio,
    pub mPresentationRect: TRect<i32>,
    pub mFullscreenBits: i32,
    pub mRefreshRate: u32,
    pub mMillisecondsPerFrame: u32,
    pub mScanLineFailCount: i32,
    pub mRedAddTable: *mut i32,
    pub mGreenAddTable: *mut i32,
    pub mBlueAddTable: *mut i32,
    pub mRedConvTable: [u32; 256],
    pub mGreenConvTable: [u32; 256],
    pub mBlueConvTable: [u32; 256],
    pub mInitialized: bool,
    pub mHWnd: *mut HWND,
    pub mIsWindowed: bool,
    pub mScreenImage: *mut DDImage,
    pub mSecondarySurfaceImage: *mut DDImage,
    pub mDDImageSet: StdSet,
    pub mVideoOnlyDraw: bool,
    pub mInitCount: u32,
    pub mCursorWidth: i32,
    pub mCursorHeight: i32,
    pub mNextCursorX: i32,
    pub mNextCursorY: i32,
    pub mCursorX: i32,
    pub mCursorY: i32,
    pub mCursorImage: *mut Image,
    pub mHasOldCursorArea: bool,
    pub mOldCursorArea: *mut IDirectDrawSurface,
    pub mNewCursorArea: *mut IDirectDrawSurface,
    pub mOldCursorAreaImage: *mut DDImage,
    pub mNewCursorAreaImage: *mut DDImage,
    pub mErrorString: StdBasicString,
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::{D3DInterface, D3DTester, DDInterface, NativeDisplay};

    #[test]
    fn check_D3DInterface_size() {
        assert_eq!(size_of::<D3DInterface>(), 88);
    }

    #[test]
    fn check_D3DTester_size() {
        assert_eq!(size_of::<D3DTester>(), 0);
    }

    #[test]
    fn check_NativeDisplay_size() {
        assert_eq!(size_of::<NativeDisplay>(), 44);
    }

    #[test]
    fn check_DDInterface_size() {
        assert_eq!(size_of::<DDInterface>(), 3408);
    }
}
