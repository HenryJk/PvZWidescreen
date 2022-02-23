use core::intrinsics::transmute;

use std::error::Error;

use crate::{memory::patch, PAD};

pub unsafe fn patch_app() -> Result<(), Box<dyn Error>> {
    // Make app width to 800 + 2 * PAD (LawnApp::LawnApp)
    patch(0x44EC12, &transmute::<i16, [u8; 2]>(800 + 2 * PAD));

    Ok(())
}
