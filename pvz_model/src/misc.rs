#![allow(non_snake_case)]

use std::{ffi::c_void, marker::PhantomData, os::raw::c_char, ptr::NonNull};

use crate::{
    stub::StdVector, Board, DrawVariation, GardenType, GridItemState, GridItemType, LawnMowerState,
    LawnMowerType, MagnetItemType, MowerHeight, PottedPlantAge, PottedPlantNeed, ScaryPotType,
    SeedType, ZombieType,
};

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

pub struct DataArrayIterator<'a, T: 'a> {
    ptr: NonNull<DataArrayItem<T>>,
    end: *const DataArrayItem<T>,
    _marker: PhantomData<&'a T>,
}

#[repr(C)]
pub struct MotionTrailFrame {
    pub mPosX: f32,
    pub mPosY: f32,
    pub mAnimTime: f32,
}

#[repr(C)]
pub struct GridItem {
    pub mApp: *const c_void,
    pub mBoard: *const Board,
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
    pub mApp: *const c_void,
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
pub struct PottedPlant {
    mSeedType: SeedType,
    mWhichZenGarden: GardenType,
    pub mX: i32,
    pub mY: i32,
    pub mFacing: i32,
    unknown0: [u8; 4],
    pub mLastWateredTime: i32,
    pub mDrawVariation: DrawVariation,
    pub mPlantAge: PottedPlantAge,
    pub mTimesFed: i32,
    pub mFeedingsPerGrow: i32,
    pub mPlantNeed: PottedPlantNeed,
    unknown1: [u8; 4],
    pub mLastNeedFulfilledTime: i32,
    pub mLastFertilizedTime: i32,
    pub mLastChocolateTime: i32,
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
