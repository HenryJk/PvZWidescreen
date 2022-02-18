use core::intrinsics::transmute;

use std::error::Error;

use iced_x86::code_asm::*;
use winapi::um::winnt::PAGE_READWRITE;

use crate::{
    memory::{change_permission, inject, patch},
    PAD,
};

pub unsafe fn patch_titlescreen() -> Result<(), Box<dyn Error>> {
    // Move IMAGE_PVZ_LOGO by PAD (TitleScreen::Draw)
    let mut code = CodeAssembler::new(32)?;
    code.add(dword_ptr(esp), PAD as i32)?;
    code.call(0x587150)?;
    code.jmp(0x48DA4C)?;
    inject(0x48DA47, code);

    // Move IMAGE_LOADBAR_DIRT by PAD (TitleScreen::Draw)
    let mut code = CodeAssembler::new(32)?;
    code.add(dword_ptr(esp), PAD as i32)?;
    code.call(0x587150)?;
    code.jmp(0x48DA72)?;
    inject(0x48DA6D, code);

    // Move IMAGE_LOADBAR_GRASS by PAD (TitleScreen::Draw)
    patch(0x48DA93, &transmute::<i16, [u8; 2]>(240 + PAD));
    patch(0x48DB00, &transmute::<i16, [u8; 2]>(240 + PAD));
    patch(0x48DB11, &transmute::<i16, [u8; 2]>(240 + PAD));

    // Move loading button by PAD (TitleScreen::Update)
    patch(0x48DECA, &transmute::<i16, [u8; 2]>(240 + PAD));

    // Move IMAGE_PLANT_SHADOW to 2000.0 (TitleScreen::Draw)
    patch(0x6794B4, &transmute::<f32, [u8; 4]>(2000.0));

    // Move IMAGE_SODROLL_CAP by PAD (TitleScreen::Draw)
    change_permission(0x6799B0, 8, PAGE_READWRITE);
    patch(0x6799B0, &transmute::<f64, [u8; 8]>(251.0 + PAD as f64));

    // Move loadbar flowers (TitleScreen::Update)
    change_permission(0x67A318, 8, PAGE_READWRITE);
    patch(0x67A318, &transmute::<f64, [u8; 8]>(225.0 + PAD as f64));

    Ok(())
}
