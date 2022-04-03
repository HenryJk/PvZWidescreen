#![allow(non_snake_case)]

use crate::{
    Color, DataArray, EmitterType, FloatParameterTrack, Image, ParticleEffect, ParticleFieldType,
    SexyVector2, TodAllocator, TodList, G_PARTICLE_DEF_ARRAY,
};

#[repr(C)]
pub struct ParticleField {
    pub mFieldType: ParticleFieldType,
    pub mX: FloatParameterTrack,
    pub mY: FloatParameterTrack,
}

#[repr(C)]
pub struct TodEmitterDefinition {
    pub mImage: *mut Image,
    pub mImageCol: i32,
    pub mImageRow: i32,
    pub mImageFrames: i32,
    pub mAnimated: i32,
    pub mParticleFlags: i32,
    pub mEmitterType: EmitterType,
    pub mName: *mut char,
    pub mOnDuration: *mut char,
    pub mSystemDuration: FloatParameterTrack,
    pub mCrossFadeDuration: FloatParameterTrack,
    pub mSpawnRate: FloatParameterTrack,
    pub mSpawnMinActive: FloatParameterTrack,
    pub mSpawnMaxActive: FloatParameterTrack,
    pub mSpawnMaxLaunched: FloatParameterTrack,
    pub mEmitterRadius: FloatParameterTrack,
    pub mEmitterOffsetX: FloatParameterTrack,
    pub mEmitterOffsetY: FloatParameterTrack,
    pub mEmitterBoxX: FloatParameterTrack,
    pub mEmitterBoxY: FloatParameterTrack,
    pub mEmitterSkewX: FloatParameterTrack,
    pub mEmitterSkewY: FloatParameterTrack,
    pub mEmitterPath: FloatParameterTrack,
    pub mParticleDuration: FloatParameterTrack,
    pub mLaunchSpeed: FloatParameterTrack,
    pub mLaunchAngle: FloatParameterTrack,
    pub mSystemRed: FloatParameterTrack,
    pub mSystemGreen: FloatParameterTrack,
    pub mSystemBlue: FloatParameterTrack,
    pub mSystemAlpha: FloatParameterTrack,
    pub mSystemBrightness: FloatParameterTrack,
    pub mParticleFields: *mut ParticleField,
    pub mParticleFieldCount: i32,
    pub mSystemFields: *mut ParticleField,
    pub mSystemFieldCount: i32,
    pub mParticleRed: FloatParameterTrack,
    pub mParticleGreen: FloatParameterTrack,
    pub mParticleBlue: FloatParameterTrack,
    pub mParticleAlpha: FloatParameterTrack,
    pub mParticleBrightness: FloatParameterTrack,
    pub mParticleSpinAngle: FloatParameterTrack,
    pub mParticleSpinSpeed: FloatParameterTrack,
    pub mParticleScale: FloatParameterTrack,
    pub mParticleStretch: FloatParameterTrack,
    pub mCollisionReflect: FloatParameterTrack,
    pub mCollisionSpin: FloatParameterTrack,
    pub mClipTop: FloatParameterTrack,
    pub mClipBottom: FloatParameterTrack,
    pub mClipLeft: FloatParameterTrack,
    pub mClipRight: FloatParameterTrack,
    pub mAnimationRate: FloatParameterTrack,
}

#[repr(C)]
pub struct TodParticleDefinition {
    pub mEmitterDefs: *const TodEmitterDefinition,
    pub mEmitterDefCount: i32,
}

#[repr(C)]
pub struct TodParticleSystem {
    pub mEffectType: ParticleEffect,
    pub mParticleDef: *mut TodParticleDefinition,
    pub mParticleHolder: *mut TodParticleHolder,
    pub mEmitterList: TodList<i32>,
    pub mDead: bool,
    pub mIsAttachment: bool,
    pub mRenderOrder: i32,
    pub mDontUpdate: bool,
}

#[repr(C)]
pub struct TodParticleEmitter {
    pub mEmitterDef: *mut TodEmitterDefinition,
    pub mParticleSystem: *mut TodParticleSystem,
    pub mParticleList: TodList<i32>,
    pub mSpawnAccum: f32,
    pub mSystemCenter: SexyVector2,
    pub mParticlesSpawned: i32,
    pub mSystemAge: i32,
    pub mSystemDuration: i32,
    pub mSystemTimeValue: f32,
    pub mSystemLastTimeValue: f32,
    pub mDead: bool,
    pub mColorOverride: Color,
    pub mExtraAdditiveDrawOverride: bool,
    pub mScaleOverride: f32,
    pub mImageOverride: *mut Image,
    pub mCrossFadeEmitterID: i32,
    pub mEmitterCrossFadeCountDown: i32,
    pub mFrameOverride: i32,
    pub mTrackInterp: [f32; 10],
    pub mSystemFieldInterp: [[f32; 2]; 4],
}

#[repr(C)]
pub struct TodParticle {
    pub mParticleEmitter: *mut TodParticleEmitter,
    pub mParticleDuration: i32,
    pub mParticleAge: i32,
    pub mParticleTimeValue: f32,
    pub mParticleLastTimeValue: f32,
    pub mAnimationTimeValue: f32,
    pub mVelocity: SexyVector2,
    pub mPosition: SexyVector2,
    pub mImageFrame: i32,
    pub mSpinPosition: f32,
    pub mSpinVelocity: f32,
    pub mCrossFadeParticleID: i32,
    pub mCrossFadeDuration: i32,
    pub mParticleInterp: [f32; 16],
    pub mParticleFieldInterp: [[f32; 2]; 4],
}

#[repr(C)]
pub struct TodParticleHolder {
    pub mParticleSystems: DataArray<TodParticleSystem>,
    pub mEmitters: DataArray<TodParticleEmitter>,
    pub mParticles: DataArray<TodParticle>,
    pub mParticleListNodeAllocator: TodAllocator,
    pub mEmitterListNodeAllocator: TodAllocator,
}

impl TodParticleDefinition {
    pub unsafe fn get(particle_effect: ParticleEffect) -> &'static Self {
        &*((*G_PARTICLE_DEF_ARRAY).offset(particle_effect as isize))
    }
}
