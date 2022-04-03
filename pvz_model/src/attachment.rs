#![allow(non_snake_case)]

use crate::{DataArray, EffectType, SexyTransform2D};

#[repr(C)]
pub struct AttachEffect {
    pub mEffectID: u32,
    pub mEffectType: EffectType,
    pub mOffset: SexyTransform2D,
    pub mDontDrawIfParentHidden: bool,
    pub mDontPropogateColor: bool,
}

#[repr(C)]
pub struct Attachment {
    pub mEffectArray: [AttachEffect; 16],
    pub mNumEffects: i32,
    pub mDead: bool,
}

#[repr(C)]
pub struct AttachmentHolder {
    mAttachments: DataArray<Attachment>,
}
