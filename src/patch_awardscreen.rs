use core::intrinsics::transmute;

use std::error::Error;

use iced_x86::code_asm::*;
use winapi::um::winnt::PAGE_READWRITE;

use crate::{
    memory::{alloc_mem, inject, patch},
    PAD,
};

pub unsafe fn patch_awardscreen() -> Result<(), Box<dyn Error>> {
    // AwardScreen::Draw
    let mut code = CodeAssembler::new(32)?;
    code.mov(dword_ptr(esp), -PAD as i32)?;
    code.mov(dword_ptr(esp + 0x8), 800 + 2 * PAD as i32)?;
    code.call(0x586D50)?;
    code.jmp(0x407681)?;
    inject(0x40767C, code);

    // AwardScreen::DrawBottom
    let mut code = CodeAssembler::new(32)?;
    code.mov(dword_ptr(esp), -PAD as i32)?;
    code.call(0x587150)?;
    code.jmp(0x406501)?;
    inject(0x4064FC, code);

    // LawnApp::ShowAwardScreen
    let mut code = CodeAssembler::new(32)?;
    code.push(0)?;
    code.push(PAD as i32)?;
    code.mov(ecx, eax)?;
    code.jmp(0x44FA97)?;
    inject(0x44fA91, code);

    // Move SeedPacket cost by PAD (DrawSeedPacket)
    let mut code = CodeAssembler::new(32)?;
    let storage_ptr = alloc_mem(4, PAGE_READWRITE) as u32;
    patch(storage_ptr, &transmute::<i16, [u8; 2]>(PAD));
    code.fld(dword_ptr(esp + 0x40))?;
    code.fiadd(dword_ptr(storage_ptr))?;
    code.fstp(dword_ptr(esp + 0x40))?;
    code.call(0x511E50)?;
    code.jmp(0x4881D6)?;
    inject(0x4881D1, code);

    Ok(())
}
