#![allow(non_snake_case)]

use crate::{
    EmitterType, FloatParameterTrack, Image, ParticleEffect, ParticleFieldType,
    G_PARTICLE_DEF_ARRAY,
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

impl TodParticleDefinition {
    pub unsafe fn get(particle_effect: ParticleEffect) -> &'static Self {
        &*((*G_PARTICLE_DEF_ARRAY).offset(particle_effect as isize))
    }
}
