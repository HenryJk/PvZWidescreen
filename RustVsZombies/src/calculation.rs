#![allow(dead_code)]

use std::ffi::CStr;

pub fn calculate_zombie_movement(zombie: &pvz::Zombie, tick_count: i32) -> f32 {
    let reanimation = unsafe {
        &(*(*(*pvz::LawnApp::instance().mEffectSystem).mReanimationHolder)
            .mReanimations
            .mBlock
            .offset(0xFFFF & zombie.mBodyReanimID as isize))
        .mItem
    };

    let mut anim_time = reanimation.mAnimTime;
    let mut pos_x = zombie.mPosX;
    for _ in 0..tick_count {
        let current_frame =
            anim_time * (reanimation.mFrameCount - 1) as f32 + reanimation.mFrameStart as f32;
        let current_frame = current_frame.floor() as isize;

        let track_def = unsafe { &*reanimation.mDefinition };
        let mut track_index = 0;
        for i in 0..track_def.mTrackCount {
            if unsafe { CStr::from_ptr((*track_def.mTracks.offset(i as isize)).mName as *const i8) }
                .to_str()
                .unwrap()
                .to_lowercase()
                == "_ground"
            {
                track_index = i as isize;
            }
        }
        let transforms = unsafe { &*track_def.mTracks.offset(track_index) }.mTransforms;

        pos_x -= unsafe {
            (*transforms.offset(current_frame + 1)).mTransX
                - (*transforms.offset(current_frame)).mTransX
        } * 0.01
            * reanimation.mAnimRate;
        anim_time += 0.01 * reanimation.mAnimRate / reanimation.mFrameCount as f32;
        if anim_time >= 1.0 {
            anim_time -= anim_time.floor();
        }
    }
    pos_x
}
