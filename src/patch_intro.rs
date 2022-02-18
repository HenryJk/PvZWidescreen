use core::intrinsics::transmute;

use std::error::Error;

use winapi::um::winnt::PAGE_READWRITE;

use crate::{
    memory::{change_permission, patch},
    PAD,
};

pub unsafe fn patch_intro() -> Result<(), Box<dyn Error>> {
    change_permission(0x6793d8, 8, PAGE_READWRITE);

    patch(0x6793d8, &transmute::<f64, [u8; 8]>(400.0 + PAD as f64));

    patch(0x4415DA, &transmute::<i16, [u8; 2]>(800 + 2 * PAD));
    patch(0x4416B0, &transmute::<i16, [u8; 2]>(400 + PAD));
    patch(0x441908, &transmute::<i16, [u8; 2]>(800 + 2 * PAD));
    patch(0x44193E, &transmute::<i16, [u8; 2]>(800 + 2 * PAD));

    patch(
        0x679738,
        &transmute::<f64, [u8; 8]>(800.0 + 2.0 * PAD as f64),
    );

    Ok(())
}
