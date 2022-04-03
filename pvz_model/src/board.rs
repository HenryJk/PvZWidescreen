#![allow(non_snake_case)]

use core::{arch::asm, ffi::c_void};

use crate::{
    AdviceType, BackgroundType, BoardResult, Challenge, Coin, CursorObject, CursorPreview,
    CutScene, DataArray, DebugTextMode, GridItem, GridSquareType, LawnApp, LawnMower, Plant,
    PlantRowType, Projectile, SeedBank, TodSmoothArray, TutorialState, Widget, Zombie, ZombieType,
};

#[repr(C)]
pub struct Board {
    pub base: Widget,
    unknown: [u8; 4],
    pub mApp: *mut LawnApp,
    pub mZombies: DataArray<Zombie>,
    pub mPlants: DataArray<Plant>,
    pub mProjectiles: DataArray<Projectile>,
    pub mCoins: DataArray<Coin>,
    pub mLawnMowers: DataArray<LawnMower>,
    pub mGridItems: DataArray<GridItem>,
    pub mCursorObject: *mut CursorObject,
    pub mCursorPreview: *mut CursorPreview,
    pub mAdvice: *const c_void,
    pub mSeedBank: *mut SeedBank,
    pub mMenuButton: *const c_void,
    pub mStoreButton: *const c_void,
    pub mIgnoreMouseUp: bool,
    pub mToolTip: *const c_void,
    pub mDebugFont: *const c_void,
    pub mCutScene: *mut CutScene,
    pub mChallenge: *mut Challenge,
    pub mPaused: bool,
    pub mGridSquareType: [[GridSquareType; 6]; 9],
    pub mGridCelLook: [[i32; 6]; 9],
    pub mGridCelOffset: [[[i32; 2]; 6]; 9],
    pub mGridCelFog: [[i32; 7]; 9],
    pub mEnableGraveStones: bool,
    pub mSpecialGraveStoneX: i32,
    pub mSpecialGraveStoneY: i32,
    pub mFogOffset: f32,
    pub mFogBlownCountdown: i32,
    pub mPlantRow: [PlantRowType; 6],
    pub mWaveRowGotLawnMowered: [i32; 6],
    pub mBonusLawnMowersRemaining: i32,
    pub mIceMinX: [i32; 6],
    pub mIceTimer: [i32; 6],
    pub mIceParticleID: [i32; 6],
    pub mRowPickingArray: [TodSmoothArray; 6],
    pub mZombiesInWave: [[ZombieType; 50]; 100],
    pub mZombieAllowed: [bool; 100],
    pub mSunCountDown: i32,
    pub mNumSunsFallen: i32,
    pub mShakeCounter: i32,
    pub mShakeAmountX: i32,
    pub mShakeAmountY: i32,
    pub mBackground: BackgroundType,
    pub mLevel: i32,
    pub mSodPosition: i32,
    pub mPrevMouseX: i32,
    pub mPrevMouseY: i32,
    pub mSunMoney: i32,
    pub mNumWaves: i32,
    pub mMainCounter: i32,
    pub mEffectCounter: i32,
    pub mDrawCount: i32,
    pub mRiseFromGraveCounter: i32,
    pub mOutOfMoneyCounter: i32,
    pub mCurrentWave: i32,
    pub mTotalSpawnedWaves: i32,
    pub mTutorialState: TutorialState,
    pub mTutorialParticleID: i32,
    pub mTutorialTimer: i32,
    pub mLastBungeeWave: i32,
    pub mZombieHealthToNextWave: i32,
    pub mZombieHealthWaveStart: i32,
    pub mZombieCountDown: i32,
    pub mZombieCountDownStart: i32,
    pub mHugeWaveCountDown: i32,
    pub mHelpDisplayed: [bool; 65],
    pub mHelpIndex: AdviceType,
    pub mFinalBossKilled: bool,
    pub mShowShovel: bool,
    pub mCoinBankFadeCount: i32,
    pub mDebugTextMode: DebugTextMode,
    pub mLevelComplete: bool,
    pub mBoardFadeOutCounter: i32,
    pub mNextSurvivalStageCounter: i32,
    pub mScoreNextMowerCounter: i32,
    pub mLevelAwardSpawned: bool,
    pub mProgressMeterWidth: i32,
    pub mFlagRaiseCounter: i32,
    pub mIceTrapCounter: i32,
    pub mBoardRandSeed: i32,
    pub mPoolSparklyParticleID: i32,
    pub mFwooshID: [[i32; 12]; 6],
    pub mFwooshCountDown: i32,
    pub mTimeStopCounter: i32,
    pub mDroppedFirstCoin: bool,
    pub mFinalWaveSoundCounter: i32,
    pub mCobCannonCursorDelayCounter: i32,
    pub mCobCannonMouseX: i32,
    pub mCobCannonMouseY: i32,
    pub mKilledYeti: bool,
    pub mMustacheMode: bool,
    pub mSuperMowerMode: bool,
    pub mFutureMode: bool,
    pub mPinataMode: bool,
    pub mDaisyMode: bool,
    pub mSukhbirMode: bool,
    pub mPrevBoardResult: BoardResult,
    pub mTriggeredLawnMowers: i32,
    pub mPlayTimeActiveLevel: i32,
    pub mPlayTimeInactiveLevel: i32,
    pub mMaxSunPlants: i32,
    pub mStartDrawTime: i32,
    pub mIntervalDrawTime: i32,
    pub mIntervalDrawCountStart: i32,
    pub mMinFPS: f32,
    pub mPreloadTime: i32,
    pub mGameIDl: i32,
    pub mGravesCleared: i32,
    pub mPlantsEaten: i32,
    pub mPlantsShoveled: i32,
    pub mCoinsCollected: i32,
    pub mDiamondsCollected: i32,
    pub mPottedPlantsCollected: i32,
    pub mChocolateCollected: i32,
}

impl Board {
    pub fn TotalZombiesHealthInWave(&self, wave: i32) -> i32 {
        let mut total = 0;
        for zombie in &self.mZombies {
            if zombie.mFromWave != wave
                || zombie.mMindControlled
                || zombie.IsDeadOrDying()
                || matches!(zombie.mZombieType, ZombieType::Bungee)
                || zombie.mRelatedZombieID != 0
            {
                continue;
            }
            total += zombie.mBodyHealth
                + zombie.mHelmHealth
                + (zombie.mShieldHealth + 2) / 5
                + zombie.mFlyingHealth
        }
        total
    }

    // button list:
    //  1 = WM_LBUTTONDOWN
    // -1 = WM_RBUTTONDOWN
    //  3 = WM_MBUTTONDOWN
    //  2 = WM_LBUTTONDBLCLK
    // -2 = WM_RBUTTONDBLCLK
    pub unsafe fn MouseDown(&mut self, x: i32, y: i32, button: i32) {
        asm!(
            "pushad",
            "push {4}",
            "push {3}",
            "push {2}",
            "mov ecx, {1}",
            "call {0}",
            "popad",
            in(reg) 0x411F20,
            in(reg) self,
            in(reg) x,
            in(reg) y,
            in(reg) button,
        )
    }

    pub unsafe fn PickZombieWaves(&mut self) {
        asm!(
            "mov edi, {1}",
            "call {0}",
            in(reg) 0x4092E0,
            in(reg) self,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::Board;

    #[test]
    fn check_Board_size() {
        assert_eq!(size_of::<Board>(), 22448);
    }
}
