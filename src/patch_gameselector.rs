use core::intrinsics::transmute;

use std::error::Error;

use iced_x86::code_asm::*;

use crate::{
    memory::{inject, patch},
    PAD, PAD_CONST_PTR,
};

pub unsafe fn patch_gameselector() -> Result<(), Box<dyn Error>> {
    // Move GameSelector by PAD (LawnApp::ShowGameSelector)
    let mut code = CodeAssembler::new(32)?;
    code.push(0)?;
    code.push(PAD as i32)?;
    code.mov(ecx, eax)?;
    code.jmp(0x44F985)?;
    inject(0x44F97F, code);

    // Move AdventureButton by PAD (unknown)
    let mut code = CodeAssembler::new(32)?;
    code.fld(dword_ptr(esp + 0x8))?;
    code.fiadd(dword_ptr(PAD_CONST_PTR))?;
    code.fstp(dword_ptr(esp + 0x8))?;
    code.sub(esp, 0x2C)?;
    code.mov(ecx, dword_ptr(eax + 0x8C))?;
    code.jmp(0x44BB29)?;
    inject(0x44BB20, code);

    // Move TodDrawStringMatrix argument by PAD (GameSelector::Draw)
    let mut code = CodeAssembler::new(32)?;
    code.push(eax)?;
    code.mov(eax, dword_ptr(esp + 0xC))?;
    code.fld(dword_ptr(eax + 0x8))?;
    code.fiadd(dword_ptr(PAD_CONST_PTR))?;
    code.fstp(dword_ptr(eax + 0x8))?;
    code.pop(eax)?;
    code.call(0x511E50)?;
    code.jmp(0x44A9CA)?;
    inject(0x44A9C5, code);

    let mut code = CodeAssembler::new(32)?;
    code.fld(dword_ptr(esp + 0x4))?;
    code.fiadd(dword_ptr(PAD_CONST_PTR))?;
    code.fstp(dword_ptr(esp + 0x4))?;
    code.call(0x512570)?;
    code.jmp(0x44AD9D)?;
    inject(0x44AD98, code);

    let mut code = CodeAssembler::new(32)?;
    code.fld(dword_ptr(esp + 0x4))?;
    code.fiadd(dword_ptr(PAD_CONST_PTR))?;
    code.fstp(dword_ptr(esp + 0x4))?;
    code.call(0x512570)?;
    code.jmp(0x44AE54)?;
    inject(0x44AE4F, code);

    let mut code = CodeAssembler::new(32)?;
    code.fld(dword_ptr(esp + 0x4))?;
    code.fiadd(dword_ptr(PAD_CONST_PTR))?;
    code.fstp(dword_ptr(esp + 0x4))?;
    code.call(0x512570)?;
    code.jmp(0x44AE15)?;
    inject(0x44AE10, code);

    let mut code = CodeAssembler::new(32)?;
    code.fld(dword_ptr(edi + 0x8))?;
    code.fiadd(dword_ptr(PAD_CONST_PTR))?;
    code.fstp(dword_ptr(edi + 0x8))?;
    code.call(0x472E40)?;
    code.fld(dword_ptr(edi + 0x8))?;
    code.fisub(dword_ptr(PAD_CONST_PTR))?;
    code.fstp(dword_ptr(edi + 0x8))?;
    code.jmp(0x44AF86)?;
    inject(0x44AF81, code);

    let mut code = CodeAssembler::new(32)?;
    code.fld(dword_ptr(edi + 0x8))?;
    code.fiadd(dword_ptr(PAD_CONST_PTR))?;
    code.fstp(dword_ptr(edi + 0x8))?;
    code.call(0x472E40)?;
    code.fld(dword_ptr(edi + 0x8))?;
    code.fisub(dword_ptr(PAD_CONST_PTR))?;
    code.fstp(dword_ptr(edi + 0x8))?;
    code.jmp(0x44AF1A)?;
    inject(0x44AF15, code);

    let mut code = CodeAssembler::new(32)?;
    code.fld(dword_ptr(edi + 0x8))?;
    code.fiadd(dword_ptr(PAD_CONST_PTR))?;
    code.fstp(dword_ptr(edi + 0x8))?;
    code.call(0x472E40)?;
    code.fld(dword_ptr(edi + 0x8))?;
    code.fisub(dword_ptr(PAD_CONST_PTR))?;
    code.fstp(dword_ptr(edi + 0x8))?;
    code.jmp(0x44AF4F)?;
    inject(0x44AF4A, code);

    let mut code = CodeAssembler::new(32)?;
    code.mov(dword_ptr(eax + 0x20), 0)?;
    code.mov(dword_ptr(eax + 0x28), 800 + 2 * PAD as i32)?;
    code.call(0x472E40)?;
    code.jmp(0x44A6E9)?;
    inject(0x44A6E4, code);

    let mut code = CodeAssembler::new(32)?;
    code.fld(dword_ptr(esp + 0x4))?;
    code.fiadd(dword_ptr(PAD_CONST_PTR))?;
    code.fstp(dword_ptr(esp + 0x4))?;
    code.call(0x512570)?;
    code.jmp(0x44AAF3)?;
    inject(0x44AAEE, code);

    let mut code = CodeAssembler::new(32)?;
    code.mov(eax, dword_ptr(eax + 0xE0))?;
    code.sub(eax, PAD as i32)?;
    code.jmp(0x44B14B)?;
    inject(0x44B145, code);

    patch(0x44B1CA, &transmute::<i16, [u8; 2]>(32 + PAD));
    patch(0x44B239, &transmute::<i16, [u8; 2]>(20 + PAD));

    Ok(())
}
