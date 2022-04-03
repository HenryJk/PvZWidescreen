#![allow(non_snake_case)]

use crate::{Board, ChallengeState, LawnApp, SeedType};

#[repr(C)]
pub struct Challenge {
    pub mApp: *mut LawnApp,
    pub mBoard: *mut Board,
    pub mBeghouledMouseCapture: bool,
    pub mBeghouledMouseDownX: i32,
    pub mBeghouledMouseDownY: i32,
    pub mBeghouledEated: [[bool; 6]; 9],
    pub mBeghouledPurcasedUpgrade: [bool; 3],
    pub mBeghouledMatchesThisMove: i32,
    pub mChallengeState: ChallengeState,
    pub mChallengeStateCounter: i32,
    pub mConveyorBeltCounter: i32,
    pub mChallengeScore: i32,
    pub mShowBowlingLine: bool,
    pub mLastConveyorSeedType: SeedType,
    pub mSurvivalStage: i32,
    pub mSlotMachineRollCount: i32,
    pub mReanimChallenge: i32,
    pub mReanimCloud: [i32; 6],
    pub mCloudCounter: [i32; 6],
    pub mChallengeGridX: i32,
    pub mChallengeGridY: i32,
    pub mScaryPotterPots: i32,
    pub mRainCounter: i32,
    pub mTreeOfWisdomTalkIndex: i32,
}
