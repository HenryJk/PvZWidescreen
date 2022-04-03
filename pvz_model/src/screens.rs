#![allow(non_snake_case)]

use core::arch::asm;

use crate::{
    AwardType, Board, ButtonWidget, ChallengePage, ChosenSeed, ChosenSeedState, CreditsPhase,
    DialogButton, GameButton, LawnApp, NewLawnButton, PerfTimer, SeedChooserState, SeedType,
    ToolTipWidget, UnlockingState, Widget,
};

#[repr(C)]
pub struct AwardScreen {
    pub base: Widget,
    pub mStartButton: *mut GameButton,
    pub mMenuButton: *mut GameButton,
    pub mApp: *mut LawnApp,
    pub mFadeInCounter: i32,
    pub mAwardType: AwardType,
}

#[repr(C)]
pub struct ChallengeScreen {
    pub base: Widget,
    unknown: [u8; 4],
    pub mBackButton: *mut NewLawnButton,
    pub mPageButton: [*mut ButtonWidget; 4],
    pub mChallengeButton: [*mut ButtonWidget; 70],
    pub mApp: *mut LawnApp,
    pub mToolTip: *mut ToolTipWidget,
    pub mPageIndex: ChallengePage,
    pub mCheatEnableChallenges: bool,
    pub mUnlockState: UnlockingState,
    pub mUnlockStateCounter: i32,
    pub mUnlockChallengeIndex: i32,
    pub mLockShakeX: f32,
    pub mLockShakeY: f32,
}

#[repr(C)]
pub struct CreditScreen {
    pub base: Widget,
    unknown0: u32,
    pub mCloseButton: *mut GameButton,
    pub mApp: *mut LawnApp,
    pub mCreditsPhase: CreditsPhase,
    pub mCreditsPhaseCounter: i32,
    pub mCreditsReanimID: i32,
    pub mFogParticleID: i32,
    pub mBlinkCountdown: i32,
    pub mMainMenuButton: *mut DialogButton,
    pub mReplayButton: *mut NewLawnButton,
    pub mOverlayWidget: *mut Widget,
    pub mDrawBrain: bool,
    pub mBrainPosX: f32,
    pub mBrainPosY: f32,
    pub mUpdateCount: i32,
    pub mDrawCount: i32,
    pub mTimerSinceStart: PerfTimer,
    pub mDontSync: bool,
    pub mCreditsPaused: bool,
    unknown1: [u8; 6],
    pub mOriginalMusicVolume: f64,
    pub mPreloaded: bool,
    unknown2: [u8; 7],
}

#[repr(C)]
pub struct SeedChooserScreen {
    pub base: Widget,
    pub mStartButton: *mut GameButton,
    pub mRandomButton: *mut GameButton,
    pub mViewLawnButton: *mut GameButton,
    pub mStoreButton: *mut GameButton,
    pub mAlmanacButton: *mut GameButton,
    pub mMenuButton: *mut GameButton,
    pub mImitaterButton: *mut GameButton,
    pub mChosenSeeds: [ChosenSeed; 53],
    pub mApp: *mut LawnApp,
    pub mBoard: *mut Board,
    pub mNumSeedsToChoose: i32,
    pub mSeedChooserAge: i32,
    pub mSeedsInFlight: i32,
    pub mSeedsInBank: i32,
    pub mToolTip: *mut ToolTipWidget,
    pub mToolTipSeed: i32,
    pub mLastMouseX: i32,
    pub mLastMouseY: i32,
    pub mChooseState: SeedChooserState,
    pub mViewLawnTime: i32,
}

impl SeedChooserScreen {
    #[inline]
    pub unsafe fn CloseSeedChooser(&mut self) {
        asm!(
            "pushad",
            "mov ebx, {1}",
            "call {0}",
            "popad",
            in(reg) 0x486D20,
            in(reg) self,
        )
    }

    pub fn ChooseSeed(&mut self, seed_type: SeedType, imitater_type: SeedType) {
        let chosen_seed: &mut ChosenSeed;
        if let SeedType::Imitater = seed_type {
            chosen_seed = &mut self.mChosenSeeds[SeedType::Imitater as usize];
            chosen_seed.mImitaterType = imitater_type;
        } else {
            chosen_seed = &mut self.mChosenSeeds[seed_type as usize];
        }
        chosen_seed.mSeedState = ChosenSeedState::InBank;
        chosen_seed.mSeedIndexInBank = self.mSeedsInBank;
        self.mSeedsInBank += 1;
    }
}
