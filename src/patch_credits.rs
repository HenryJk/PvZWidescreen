use core::intrinsics::transmute;

use std::error::Error;

use iced_x86::code_asm::*;

use crate::{
    memory::{inject, patch},
    PAD, PAD_CONST_PTR,
};

unsafe fn patch_draw_image_f_call(address: u32) -> Result<(), Box<dyn Error>> {
    const DRAWIMAGEF_FN_PTR: u64 = 0x587630;

    let mut code = CodeAssembler::new(32)?;
    code.fld(dword_ptr(esp))?;
    code.fisub(dword_ptr(PAD_CONST_PTR))?;
    code.fstp(dword_ptr(esp))?;
    code.call(DRAWIMAGEF_FN_PTR)?;
    code.jmp(address as u64 + 5)?;
    inject(address, code);

    Ok(())
}

unsafe fn patch_fill_rect_call(address: u32) -> Result<(), Box<dyn Error>> {
    const FILLRECT_FN_PTR: u64 = 0x586D50;

    let mut code = CodeAssembler::new(32)?;
    code.mov(dword_ptr(esp), -PAD as i32)?;
    code.call(FILLRECT_FN_PTR)?;
    code.jmp(address as u64 + 5)?;
    inject(address, code);

    Ok(())
}

unsafe fn patch_drawdisco_call(address: u32) -> Result<(), Box<dyn Error>> {
    const DRAWDISCO_FN_PTR: u64 = 0x434F20;

    let mut code = CodeAssembler::new(32)?;
    code.fld(dword_ptr(esp))?;
    code.fiadd(dword_ptr(PAD_CONST_PTR))?;
    code.fstp(dword_ptr(esp))?;
    code.call(DRAWDISCO_FN_PTR)?;
    code.jmp(address as u64 + 5)?;
    inject(address, code);

    Ok(())
}

pub unsafe fn patch_credits() -> Result<(), Box<dyn Error>> {
    patch_fill_rect_call(0x435BBE)?;
    patch_fill_rect_call(0x435CAA)?;

    let mut code = CodeAssembler::new(32)?;
    code.push(PAD as i32)?;
    code.mov(ecx, eax)?;
    code.call(edx)?;
    code.jmp(0x44FBA4)?;
    inject(0x44FB9E, code);

    const DRAWIMAGEF_CALL_ADDRESS: [u32; 6] =
        [0x435F8C, 0x436029, 0x4360D1, 0x436298, 0x436353, 0x43641E];
    for address in DRAWIMAGEF_CALL_ADDRESS {
        patch_draw_image_f_call(address)?;
    }

    let mut code = CodeAssembler::new(32)?;
    code.fiadd(dword_ptr(PAD_CONST_PTR))?;
    code.fadd(qword_ptr(0x679FF0))?;
    code.jmp(0x436649)?;
    inject(0x436643, code);

    patch_drawdisco_call(0x436707)?;
    patch_drawdisco_call(0x43673C)?;

    patch(0x4350F1, &transmute::<i16, [u8; 2]>(800 + 2 * PAD));

    let mut code = CodeAssembler::new(32)?;
    code.sub(dword_ptr(esp + 0x4), PAD as i32)?;
    code.call(0x587E80)?;
    code.jmp(0x435482)?;
    inject(0x43547D, code);

    // patch(0x43643E, &[0x83, 0xC4, 0x08, 0x90, 0x90]);

    Ok(())
}
