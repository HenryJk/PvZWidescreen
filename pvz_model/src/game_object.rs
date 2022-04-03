#![allow(non_snake_case)]

use core::arch::asm;

use crate::{
    Board, CoinMotion, CoinType, CursorType, HelmType, LawnApp, MagnetItem, PlantState,
    PlantSubClass, PottedPlant, ProjectileMotion, ProjectileType, SeedType, TRect, ZombieHeight,
    ZombiePhase, ZombieType,
};

#[repr(C)]
pub struct GameObject {
    pub mApp: *mut LawnApp,
    pub mBoard: *mut Board,
    pub mX: i32,
    pub mY: i32,
    pub mWidth: i32,
    pub mHeight: i32,
    pub mVisible: bool,
    pub mRow: i32,
    pub mRenderOrder: i32,
}

#[repr(C)]
pub struct Coin {
    pub base: GameObject,
    pub mPosX: f32,
    pub mPosY: f32,
    pub mVelX: f32,
    pub mVelY: f32,
    pub mScale: f32,
    pub mDead: bool,
    pub mFadeCount: i32,
    pub mCollectX: f32,
    pub mCollectY: f32,
    pub mGroundY: i32,
    pub mCoinAge: i32,
    pub mIsBeingCollected: bool,
    pub mDisappearCounter: i32,
    pub mType: CoinType,
    pub mCoinMotion: CoinMotion,
    pub mAttachmentID: i32,
    pub mCollectionDistance: f32,
    pub mUsableSeedType: SeedType,
    unknown: [u8; 4],
    pub mPottedPlantSpec: PottedPlant,
    pub mNeedsBouncyArrow: bool,
    pub mHasBouncyArrow: bool,
    pub mHitGround: bool,
    pub mTimesDropped: i32,
}

#[repr(C)]
pub struct CursorObject {
    pub base: GameObject,
    pub mSeedBankIndex: i32,
    pub mType: SeedType,
    pub mImitaterType: SeedType,
    pub mCursorType: CursorType,
    pub mCoinID: i32,
    pub mGlovePlantID: i32,
    pub mDuplicatorPlantID: i32,
    pub mCobCannonPlantID: i32,
    pub mHammerDownCounter: i32,
    pub mReanimCursorID: i32,
}

#[repr(C)]
pub struct CursorPreview {
    pub base: GameObject,
    pub mGridX: i32,
    pub mGridY: i32,
}

#[repr(C)]
pub struct Plant {
    pub base: GameObject,
    pub mSeedType: SeedType,
    pub mPlantCol: i32,
    pub mAnimCounter: i32,
    pub mFrame: i32,
    pub mFrameLength: i32,
    pub mNumFrames: i32,
    pub mState: PlantState,
    pub mPlantHealth: i32,
    pub mPlantMaxHealth: i32,
    pub mSubclass: PlantSubClass,
    pub mDisappearCountdown: i32,
    pub mDoSpecialCountdown: i32,
    pub mStateCountdown: i32,
    pub mLaunchCounter: i32,
    pub mLaunchRate: i32,
    pub mPlantRect: TRect<i32>,
    pub mPlantAttackRect: TRect<i32>,
    pub mTargetX: i32,
    pub mTargetY: i32,
    pub mStartRow: i32,
    pub mParticleID: i32,
    pub mShootingCounter: i32,
    pub mBodyReanimID: i32,
    pub mHeadReanimID: i32,
    pub mHeadReanimID2: i32,
    pub mHeadReanimID3: i32,
    pub mBlinkReanimID: i32,
    pub mLightReanimID: i32,
    pub mSleepingReanimID: i32,
    pub mBlinkCountdown: i32,
    pub mRecentlyEatenCountdown: i32,
    pub mEatenFlashCountdown: i32,
    pub mBeghouledFlashCountdown: i32,
    pub mShakeOffsetX: f32,
    pub mShakeOffsetY: f32,
    pub mMagnetItems: [MagnetItem; 5],
    pub mTargetZombieID: i32, // For squash
    pub mWakeUpCounter: i32,
    pub mInvisibilityLevel: i32,
    pub mImitaterType: SeedType,
    pub mOrientation: i32,
    unknown: [u8; 1],
    pub mDead: bool,
    pub mSquished: bool,
    pub mIsAsleep: bool,
    pub mIsOnBoard: bool,
    pub mHighlighted: bool,
}

#[repr(C)]
pub struct Projectile {
    pub base: GameObject,
    pub mFrame: i32,
    pub mNumFrames: i32,
    pub mAnimCounter: i32,
    pub mPosX: f32,
    pub mPosY: f32,
    pub mPosZ: f32,
    pub mVelX: f32,
    pub mVelY: f32,
    pub mVelZ: f32,
    pub mAccZ: f32,
    pub mShadowY: f32,
    pub mDead: bool,
    pub mAnimTicksPerFrame: i32,
    pub mMotionType: ProjectileMotion,
    pub mProjectileType: ProjectileType,
    pub mProjectileAge: i32,
    pub mClickBackoffCounter: i32,
    pub mRotation: f32,
    pub mRotationSpeed: f32,
    pub mOnHighGround: bool,
    pub mDamageRangeFlags: i32,
    pub mHitTorchwoodGridX: i32,
    pub mAttachmentID: i32,
    pub mCobTargetX: f32,
    pub mCobTargetRow: i32,
    pub mTargetZombieID: i32,
    pub mLastPortalX: i32,
}

#[repr(C)]
pub struct SeedBank {
    pub base: GameObject,
    pub mNumPackets: i32,
    pub mSeedPackets: [SeedPacket; 10],
    pub mCutSceneDarken: i32,
    pub mConveyorBeltCounter: i32,
}

#[repr(C)]
pub struct SeedPacket {
    pub base: GameObject,
    pub mRefreshCounter: i32,
    pub mRefreshTime: i32,
    pub mIndex: i32,
    pub mOffsetX: i32,
    pub mPacketType: SeedType,
    pub mImitaterType: SeedType,
    pub mSlotMachineCountDown: i32,
    pub mSlotMachiningNextSeed: SeedType,
    pub mSlotMachiningPosition: f32,
    pub mActive: bool,
    pub mRefreshing: bool,
    pub mTimesUsed: i32,
}

#[repr(C)]
pub struct Zombie {
    pub base: GameObject,
    pub mZombieType: ZombieType,
    pub mZombiePhase: ZombiePhase,
    pub mPosX: f32,
    pub mPosY: f32,
    pub mVelX: f32,
    pub mAnimCounter: i32,
    pub mGroanCounter: i32,
    pub mAnimTicksPerFrame: i32,
    pub mAnimFrames: i32,
    pub mFrame: i32,
    pub mPrevFrame: i32,
    pub mVariant: bool,
    pub mIsEating: bool,
    pub mJustGotShotCounter: i32,
    pub mShieldJustGotShotCounter: i32,
    pub mShieldRecoilCounter: i32,
    pub mZombieAge: i32,
    pub mZombieHeight: ZombieHeight,
    pub mPhaseCounter: i32,
    pub mFromWave: i32,
    pub mDroppedLoot: bool,
    pub mZombieFade: i32,
    pub mFlatTires: bool,
    pub mUseLadderCol: i32,
    pub mTargetCol: i32,
    pub mAltitude: f32,
    pub mHitUmbrella: bool,
    pub mZombieRect: TRect<i32>,
    pub mZombieAttackRect: TRect<i32>,
    pub mChilledCounter: i32,
    pub mButteredCounter: i32,
    pub mIceTrapCounter: i32,
    pub mMindControlled: bool,
    pub mBlowingAway: bool,
    pub mHasHead: bool,
    pub mHasArm: bool,
    pub mHasObject: bool,
    pub mInPool: bool,
    pub mOnHighGround: bool,
    pub mYuckyFace: bool,
    pub mYuckyFaceCounter: i32,
    pub mHelmType: HelmType,
    pub mBodyHealth: i32,
    pub mBodyMaxHealth: i32,
    pub mHelmHealth: i32,
    pub mHelmMaxHealth: i32,
    pub mShieldType: SeedType,
    pub mShieldHealth: i32,
    pub mShieldMaxHealth: i32,
    pub mFlyingHealth: i32,
    pub mFlyingMaxHealth: i32,
    pub mDead: bool,
    pub mRelatedZombieID: i32,
    pub mFollowerZombieID: [i32; 4],
    pub mPlayingSong: bool,
    pub mParticleOffsetX: i32,
    pub mParticleOffsetY: i32,
    pub mAttachmentID: i32,
    pub mSummonCounter: i32,
    pub mBodyReanimID: i32,
    pub mScaleZombie: f32,
    pub mVelZ: f32,
    pub mOrginalAnimRate: f32,
    pub mTargetPlantID: i32,
    pub mBossMode: i32,
    pub mTargetRow: i32,
    pub mBossBungeeCounter: i32,
    pub mBossStompCounter: i32,
    pub mBossHeadCounter: i32,
    pub mBossFireBallReanimID: i32,
    pub mSpecialHeadReanimID: i32,
    pub mFireballRow: i32,
    pub mIsFireBall: bool,
    pub mMoweredReanimID: i32,
    pub mLastPortalX: i32,
}

impl Plant {
    pub unsafe fn CobCannonFire(&mut self, x: i32, y: i32) {
        asm!(
            "pushad",
            "push {3}",
            "push {2}",
            "mov eax, {1}",
            "call {0}",
            "popad",
            in(reg) 0x466D50,
            in(reg) self,
            in(reg) x,
            in(reg) y
        )
    }
}

impl Zombie {
    pub fn IsDeadOrDying(&self) -> bool {
        self.mDead
            || self.mZombiePhase == ZombiePhase::ZombieDying
            || self.mZombiePhase == ZombiePhase::ZombieBurned
            || self.mZombiePhase == ZombiePhase::ZombieMowered
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::{
        Coin, CursorObject, CursorPreview, GameObject, Plant, Projectile, SeedBank, SeedPacket,
        Zombie,
    };

    #[test]
    fn check_GameObject_size() {
        assert_eq!(size_of::<GameObject>(), 36);
    }

    #[test]
    fn check_Coin_size() {
        assert_eq!(size_of::<Coin>(), 208);
    }

    #[test]
    fn check_CursorObject_size() {
        assert_eq!(size_of::<CursorObject>(), 76);
    }

    #[test]
    fn check_CursorPreview_size() {
        assert_eq!(size_of::<CursorPreview>(), 44);
    }

    #[test]
    fn check_Plant_size() {
        assert_eq!(size_of::<Plant>(), 328);
    }

    #[test]
    fn check_Projectile_size() {
        assert_eq!(size_of::<Projectile>(), 144);
    }

    #[test]
    fn check_SeedBank_size() {
        assert_eq!(size_of::<SeedBank>(), 848);
    }

    #[test]
    fn check_SeedPacket_size() {
        assert_eq!(size_of::<SeedPacket>(), 80);
    }

    #[test]
    fn check_Zombie_size() {
        assert_eq!(size_of::<Zombie>(), 344);
    }
}
