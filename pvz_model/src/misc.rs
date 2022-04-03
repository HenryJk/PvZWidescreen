#![allow(non_snake_case)]

use std::{ffi::c_void, marker::PhantomData, os::raw::c_char, ptr::NonNull};

use crate::{
    stub::{StdBasicString, StdVector},
    AttachmentHolder, Board, DrawVariation, GardenType, GridItemState, GridItemType, Image,
    LawnApp, LawnMowerState, LawnMowerType, MagnetItemType, MemoryImage, MowerHeight,
    PottedPlantAge, PottedPlantNeed, ReanimationHolder, ScaryPotType, SeedType, TodCurves,
    TodParticleHolder, TrailHolder, Widget, ZombieType,
};

pub type Undefined1 = char;

#[repr(C)]
pub struct ButtonListener {
    unknown: [u8; 4],
}

#[repr(C)]
pub struct GameButton {
    pub mApp: *mut LawnApp,
    pub mParentWidget: *mut Widget,
    pub mX: i32,
    pub mY: i32,
    pub mWidth: i32,
    pub mHeight: i32,
    pub mIsOver: bool,
    pub mIsDown: bool,
    pub mDisabled: bool,
    pub mColors: [Color; 6],
    pub mId: i32,
    pub mLabel: [char; 28],
    pub mLabelJustify: i32,
    pub mFont: *mut Font,
    pub mButtonImage: *mut Image,
    pub mOverImage: *mut Image,
    pub mDownImage: *mut Image,
    pub mDisabledImage: *mut Image,
    pub mOverOverlayImage: *mut Image,
    pub mNormalRect: TRect<i32>,
    pub mOverRect: TRect<i32>,
    pub mDownRect: TRect<i32>,
    pub mDisabledRect: TRect<i32>,
    pub mInverted: bool,
    pub mBtnNoDraw: bool,
    pub mFrameNoDraw: bool,
    unknown0: [u8; 4],
    pub mOverAlpha: f64,
    pub mOverAlphaSpeed: f64,
    pub mOverAlphaFadeInSpeed: f64,
    pub mDrawStoneButton: bool,
    pub mTextOffsetX: i32,
    pub mTextOffsetY: i32,
    pub mButtonOffsetX: i32,
    pub mButtonOffsetY: i32,
    unknown1: [u8; 4],
}

#[repr(C)]
pub struct Buffer {
    unknown: [u8; 4],
    pub mData: StdVector,
    pub mDataBitSize: i32,
    pub mReadBitPos: i32,
    pub mWriteBitPos: i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Color {
    pub mRed: i32,
    pub mGreen: i32,
    pub mBlue: i32,
    pub mAlpha: i32,
}

#[repr(C)]
pub struct DataArrayItem<T> {
    pub mItem: T,
    pub mID: u32,
}

#[repr(C)]
pub struct DataArray<T> {
    pub mBlock: *mut DataArrayItem<T>,
    pub mMaxUsedCount: u32,
    pub mMaxSize: u32,
    pub mFreeListHead: u32,
    pub mSize: u32,
    pub mNextKey: u32,
    pub mName: *const c_char,
}

#[repr(C)]
pub struct TodListNode<T> {
    pub mValue: T,
    pub mNext: *mut TodListNode<T>,
    pub mPrev: *mut TodListNode<T>,
}

#[repr(C)]
pub struct TodList<T> {
    pub mHead: *mut TodListNode<T>,
    pub mTail: *mut TodListNode<T>,
    pub mSize: i32,
    pub mpAllocator: *mut TodAllocator,
}

#[repr(C)]
pub struct ChosenSeed {
    pub mX: i32,
    pub mY: i32,
    pub mTimeStartMotion: i32,
    pub mTimeEndMotion: i32,
    pub mStartX: i32,
    pub mStartY: i32,
    pub mEndX: i32,
    pub mEndY: i32,
    pub mSeedType: SeedType,
    pub mSeedState: crate::ChosenSeedState,
    pub mSeedIndexInBank: i32,
    pub mRefreshing: bool,
    pub mRefreshCounter: i32,
    pub mImitaterType: SeedType,
    pub mCrazyDavePicked: bool,
}

#[repr(C)]
pub struct Font {
    unknown: [u8; 4],
    pub mAscent: i32,
    pub mAscentPadding: i32,
    pub mHeight: i32,
    pub mLineSpacingOffset: i32,
}

#[repr(C)]
pub struct MotionTrailFrame {
    pub mPosX: f32,
    pub mPosY: f32,
    pub mAnimTime: f32,
}

#[repr(C)]
pub struct GridItem {
    pub mApp: *mut LawnApp,
    pub mBoard: *mut Board,
    pub mGridItemType: GridItemType,
    pub mGridItemState: GridItemState,
    pub mGridX: i32,
    pub mGridY: i32,
    pub mGridItemCounter: i32,
    pub mRenderOrder: i32,
    pub mDead: bool,
    pub mPosX: f32,
    pub mPosY: f32,
    pub mGoalX: f32,
    pub mGoalY: f32,
    pub mGridItemReanimID: i32,
    pub mGridItemParticleID: i32,
    pub mZombieType: ZombieType,
    pub mSeedType: SeedType,
    pub mScaryPotType: ScaryPotType,
    pub mHighlighted: bool,
    pub mTransparentCounter: i32,
    pub mSunCount: i32,
    pub mMotionTrailFrames: [MotionTrailFrame; 12],
    pub mMotionTrailCount: i32,
}

#[repr(C)]
pub struct MagnetItem {
    pub mPosX: f32,
    pub mPosY: f32,
    pub mDestOffsetX: f32,
    pub mDestOffsetY: f32,
    pub mItemType: MagnetItemType,
}

#[repr(C)]
pub struct LawnMower {
    pub mApp: *mut LawnApp,
    pub mBoard: *mut Board,
    pub mPosX: f32,
    pub mPosY: f32,
    pub mRenderOrder: i32,
    pub mRow: i32,
    pub mAnimTicksPerFrame: i32,
    pub mReanimID: i32,
    pub mChompCounter: i32,
    pub mRollingInCounter: i32,
    pub mSquishedCounter: i32,
    pub mMowerState: LawnMowerState,
    pub mDead: bool,
    pub mVisible: bool,
    pub mMowerType: LawnMowerType,
    pub mAltitude: f32,
    pub mMowerHeight: MowerHeight,
    pub mLastPortalX: i32,
}

#[repr(C)]
pub struct PerfTimer {
    pub mStart: i64,
    pub mDuration: f64,
    pub mRunning: bool,
    unknown: [u8; 7],
}

#[repr(C)]
pub struct PottedPlant {
    mSeedType: SeedType,
    mWhichZenGarden: GardenType,
    pub mX: i32,
    pub mY: i32,
    pub mFacing: i32,
    unknown0: [u8; 4],
    pub mLastWateredTime: i64,
    pub mDrawVariation: DrawVariation,
    pub mPlantAge: PottedPlantAge,
    pub mTimesFed: i32,
    pub mFeedingsPerGrow: i32,
    pub mPlantNeed: PottedPlantNeed,
    unknown1: [u8; 4],
    pub mLastNeedFulfilledTime: i64,
    pub mLastFertilizedTime: i64,
    pub mLastChocolateTime: i64,
    pub mFutureAttribute: [i32; 1],
    unknown2: [u8; 4],
}

#[repr(C)]
pub struct Ratio {
    pub mNumerator: i32,
    pub mDenominator: i32,
}

#[repr(C)]
pub struct TodSmoothArray {
    pub mItem: i32,
    pub mWeight: f32,
    pub mLastPicked: f32,
    pub mSecondLastPicked: f32,
}

#[repr(C)]
pub struct TRect<T> {
    pub mX: T,
    pub mY: T,
    pub mWidth: T,
    pub mHeight: T,
}

#[repr(C)]
pub struct TypingCheck {
    pub mPhrase: StdBasicString,
    pub mRecentTyping: StdBasicString,
}

#[repr(C)]
pub struct FloatParameterTrackNode {
    pub mTime: f32,
    pub mLowValue: f32,
    pub mHighValue: f32,
    pub mCurveType: TodCurves,
    pub mDistribution: TodCurves,
}

#[repr(C)]
pub struct FloatParameterTrack {
    pub mNodes: *mut FloatParameterTrackNode,
    pub mCountNodes: i32,
}

#[repr(C)]
pub struct ToolTipWidget {
    pub mTitle: StdBasicString,
    pub mLabel: StdBasicString,
    pub mWarningText: StdBasicString,
    pub mX: i32,
    pub mY: i32,
    pub mWidth: i32,
    pub mHeight: i32,
    pub mVisible: bool,
    pub mCenter: bool,
    pub mMinLeft: i32,
    pub mMaxBottom: i32,
    pub mGetsLinesWidth: i32,
    pub mWarningFlashCounter: i32,
}

#[repr(C)]
pub struct EffectSystem {
    pub mParticleHolder: *mut TodParticleHolder,
    pub mTrailHolder: *mut TrailHolder,
    pub mReanimationHolder: *mut ReanimationHolder,
    pub mAttachmentHolder: *mut AttachmentHolder,
}

#[repr(C)]
pub struct PoolEffect {
    pub mCausticGrayscaleImage: *mut u8,
    pub mCausticImage: *mut MemoryImage,
    pub mApp: *mut LawnApp,
    pub mPoolCounter: i32,
}

#[repr(C)]
pub struct TodAllocator {
    pub mFreeList: *mut c_void,
    pub mBlockList: *mut c_void,
    pub mGrowCount: i32,
    pub mTotalItems: i32,
    pub mItemSize: i32,
}

#[repr(C)]
pub struct SexyVector2 {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
pub struct SexyVector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
pub struct ZenGarden {
    pub mApp: *mut LawnApp,
    pub mBoard: *mut Board,
    pub mGardenType: GardenType,
}

impl Color {
    pub fn new(hexcode: u32) -> Self {
        Color {
            mRed: (hexcode >> 24) as i32,
            mGreen: ((hexcode >> 16) & 0xFF) as i32,
            mBlue: ((hexcode >> 8) & 0xFF) as i32,
            mAlpha: (hexcode & 0xFF) as i32,
        }
    }
}

pub struct DataArrayIterator<'a, T: 'a> {
    ptr: NonNull<DataArrayItem<T>>,
    end: *const DataArrayItem<T>,
    _marker: PhantomData<&'a T>,
}

pub struct DataArrayMutIterator<'a, T: 'a> {
    ptr: NonNull<DataArrayItem<T>>,
    end: *const DataArrayItem<T>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for DataArrayIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        while self.ptr.as_ptr() as *const DataArrayItem<T> != self.end {
            if (unsafe { self.ptr.as_ref() }.mID & 0xFFFF0000) > 0 {
                break;
            }
            self.ptr = unsafe { NonNull::new(self.ptr.as_ptr().offset(1)).unwrap() };
        }
        let result = if self.ptr.as_ptr() as *const DataArrayItem<T> != self.end {
            Some(&unsafe { self.ptr.as_ref() }.mItem)
        } else {
            None
        };
        self.ptr = unsafe { NonNull::new(self.ptr.as_ptr().offset(1)).unwrap() };
        result
    }
}

impl<'a, T> Iterator for DataArrayMutIterator<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        while self.ptr.as_ptr() as *const DataArrayItem<T> != self.end {
            if (unsafe { self.ptr.as_ref() }.mID & 0xFFFF0000) > 0 {
                break;
            }
            self.ptr = unsafe { NonNull::new(self.ptr.as_ptr().offset(1)).unwrap() };
        }
        let result = if self.ptr.as_ptr() as *const DataArrayItem<T> != self.end {
            Some(&mut unsafe { self.ptr.as_mut() }.mItem)
        } else {
            None
        };
        self.ptr = unsafe { NonNull::new(self.ptr.as_ptr().offset(1)).unwrap() };
        result
    }
}

impl<'a, T> IntoIterator for &'a DataArray<T> {
    type Item = &'a T;
    type IntoIter = DataArrayIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        DataArrayIterator {
            ptr: NonNull::new(self.mBlock).unwrap(),
            end: unsafe { self.mBlock.offset(self.mMaxUsedCount as isize) },
            _marker: PhantomData,
        }
    }
}

impl<'a, T> IntoIterator for &'a mut DataArray<T> {
    type Item = &'a mut T;
    type IntoIter = DataArrayMutIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        DataArrayMutIterator {
            ptr: NonNull::new(self.mBlock).unwrap(),
            end: unsafe { self.mBlock.offset(self.mMaxUsedCount as isize) },
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::{
        Buffer, ButtonListener, Color, Font, GridItem, LawnMower, MagnetItem, MotionTrailFrame,
        PottedPlant, Ratio, TodSmoothArray, TypingCheck,
    };

    #[test]
    fn check_ButtonListener_size() {
        assert_eq!(size_of::<ButtonListener>(), 4);
    }

    #[test]
    fn check_Buffer_size() {
        assert_eq!(size_of::<Buffer>(), 32);
    }

    #[test]
    fn check_Color_size() {
        assert_eq!(size_of::<Color>(), 16);
    }

    #[test]
    fn check_Font_size() {
        assert_eq!(size_of::<Font>(), 20);
    }

    #[test]
    fn check_MotionTrailFrame_size() {
        assert_eq!(size_of::<MotionTrailFrame>(), 12);
    }

    #[test]
    fn check_GridItem_size() {
        assert_eq!(size_of::<GridItem>(), 232);
    }

    #[test]
    fn check_MagnetItem_size() {
        assert_eq!(size_of::<MagnetItem>(), 20);
    }

    #[test]
    fn check_LawnMower_size() {
        assert_eq!(size_of::<LawnMower>(), 68);
    }

    #[test]
    fn check_PottedPlant_size() {
        assert_eq!(size_of::<PottedPlant>(), 88);
    }

    #[test]
    fn check_Ratio_size() {
        assert_eq!(size_of::<Ratio>(), 8);
    }

    #[test]
    fn check_TodSmoothArray_size() {
        assert_eq!(size_of::<TodSmoothArray>(), 16);
    }

    #[test]
    fn check_TypingCheck_size() {
        assert_eq!(size_of::<TypingCheck>(), 56);
    }
}
