#![allow(non_snake_case)]

use crate::{Board, LawnApp};

#[repr(C)]
pub struct CutScene {
    pub mApp: *mut LawnApp,
    pub mBoard: *mut Board,
    pub mCutsceneTime: i32,
    pub mSodTime: i32,
    pub mGraveStoneTime: i32,
    pub mReadySetPlantTime: i32,
    pub mFogTime: i32,
    pub mBossTime: i32,
    pub mCrazyDaveTime: i32,
    pub mLawnMowerTime: i32,
    pub mCrazyDaveDialogStart: i32,
    pub mSeedChoosing: bool,
    pub mZombiesWonReanimID: i32,
    pub mPreloaded: bool,
    pub mPlacedZombies: bool,
    pub mPlacedLawnItems: bool,
}
