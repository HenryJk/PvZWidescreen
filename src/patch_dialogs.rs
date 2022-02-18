use core::intrinsics::transmute;

use std::error::Error;

use crate::{memory::patch, PAD};

pub unsafe fn patch_dialogs() -> Result<(), Box<dyn Error>> {
    // (LawnApp::DoContinueDialog)
    patch(0x45027D, &transmute::<i16, [u8; 2]>(800 + 2 * PAD));

    // (LawnApp::ConfirmQuit)
    patch(0x453161, &transmute::<i16, [u8; 2]>(800 + 2 * PAD));

    // (LawnApp::DoNewOptions)
    patch(0x450137, &transmute::<i16, [u8; 2]>(800 + 2 * PAD));

    // (LawnApp::DoPauseDialog)
    patch(0x45047D, &transmute::<i16, [u8; 2]>(800 + 2 * PAD));

    // Imitater dialog (SeedChooserScreen::MouseDown)
    patch(0x486A2C, &transmute::<i16, [u8; 2]>(800 + 2 * PAD));

    // (LawnApp::NewDialog)
    patch(0x451602, &transmute::<i16, [u8; 2]>(800 + 2 * PAD));

    // (LawnApp::DoUserDialog)
    patch(0x4508EB, &transmute::<i16, [u8; 2]>(800 + 2 * PAD));

    // (LawnApp::DoCreateUserDialog)
    patch(0x450A7C, &transmute::<i16, [u8; 2]>(800 + 2 * PAD));

    // (LawnApp::DoRenameUserDialog)
    patch(0x4511EE, &transmute::<i16, [u8; 2]>(800 + 2 * PAD));

    Ok(())
}
