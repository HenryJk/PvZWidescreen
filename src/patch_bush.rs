use std::error::Error;

use iced_x86::code_asm::*;
use winapi::um::winnt::PAGE_READWRITE;

use crate::{
    memory::{alloc_mem, inject, patch},
    POLE_NIGHT_PTR, POLE_PTR,
};

pub unsafe fn patch_bush() -> Result<(), Box<dyn Error>> {
    // Load IMAGE_POLE (Sexy::ExtractDelayLoad_Background5Resources)
    let storage_ptr = alloc_mem(0x100, PAGE_READWRITE) as u32;
    const POLE_NAME: &str = "IMAGE_POLE";
    patch(storage_ptr + 0x20, POLE_NAME.as_bytes());
    let mut code = CodeAssembler::new(32)?;
    code.push(POLE_NAME.len() as u32)?;
    code.push(storage_ptr + 0x20)?;
    code.mov(ecx, storage_ptr)?;
    code.mov(dword_ptr(storage_ptr + 0x18), 0xf)?;
    code.mov(dword_ptr(storage_ptr + 0x14), 0)?;
    code.mov(byte_ptr(storage_ptr + 0x4), 0)?;
    code.call(0x404330)?;
    code.mov(edx, dword_ptr(edi))?;
    code.mov(edx, dword_ptr(edx + 0x40))?;
    code.push(storage_ptr)?;
    code.push(storage_ptr + 0x80)?;
    code.mov(ecx, edi)?;
    code.call(edx)?;
    code.mov(ecx, eax)?;
    code.call(0x59A990)?;
    code.mov(esi, storage_ptr + 0x80)?;
    code.mov(dword_ptr(POLE_PTR), eax)?;
    code.call(0x59A8D0)?;
    code.mov(al, 1)?;
    code.mov(ecx, dword_ptr(ebp - 0xC))?;
    code.jmp(0x475925)?;
    inject(0x475920, code);

    // Load IMAGE_POLE_NIGHT (Sexy::ExtractDelayLoad_Background6Resources)
    let storage_ptr = alloc_mem(0x100, PAGE_READWRITE) as u32;
    const POLE_NIGHT_NAME: &str = "IMAGE_POLE_NIGHT";
    patch(storage_ptr + 0x20, POLE_NIGHT_NAME.as_bytes());
    let mut code = CodeAssembler::new(32)?;
    code.push(POLE_NIGHT_NAME.len() as u32)?;
    code.push(storage_ptr + 0x20)?;
    code.mov(ecx, storage_ptr)?;
    code.mov(dword_ptr(storage_ptr + 0x18), 0xf)?;
    code.mov(dword_ptr(storage_ptr + 0x14), 0)?;
    code.mov(byte_ptr(storage_ptr + 0x4), 0)?;
    code.call(0x404330)?;
    code.mov(edx, dword_ptr(edi))?;
    code.mov(edx, dword_ptr(edx + 0x40))?;
    code.push(storage_ptr)?;
    code.push(storage_ptr + 0x80)?;
    code.mov(ecx, edi)?;
    code.call(edx)?;
    code.mov(ecx, eax)?;
    code.call(0x59A990)?;
    code.mov(esi, storage_ptr + 0x80)?;
    code.mov(dword_ptr(POLE_NIGHT_PTR), eax)?;
    code.call(0x59A8D0)?;
    code.mov(al, 1)?;
    code.mov(ecx, dword_ptr(ebp - 0xC))?;
    code.jmp(0x475A35)?;
    inject(0x475A30, code);

    // Put Obstruction pole on draw queue (Board::DrawGameObjects)
    let mut code = CodeAssembler::new(32)?;
    let mut endif = code.create_label();
    let mut case_bg_5 = code.create_label();
    let mut case_bg_6 = code.create_label();
    code.mov(esi, dword_ptr(esp + 0x14))?;
    code.mov(esi, dword_ptr(esi + 0x554C))?;
    code.cmp(esi, 4)?;
    code.je(case_bg_5)?;
    code.cmp(esi, 5)?;
    code.je(case_bg_6)?;
    code.mov(esi, dword_ptr(esp + 0x4))?;
    code.jmp(endif)?;
    code.set_label(&mut case_bg_5)?;
    code.mov(esi, dword_ptr(esp + 0x4))?;
    code.mov(dword_ptr(esi), 0x19)?;
    code.mov(dword_ptr(esi + 0x4), 400_001)?;
    code.mov(dword_ptr(esi + 0x8), POLE_PTR)?;
    code.add(dword_ptr(esp + 0x4), 0xC)?;
    code.add(dword_ptr(esp + 0x8), 1)?;
    code.add(esi, 0xC)?;
    code.inc(ecx)?;
    code.inc(edi)?;
    code.jmp(endif)?;
    code.set_label(&mut case_bg_6)?;
    code.mov(esi, dword_ptr(esp + 0x4))?;
    code.mov(dword_ptr(esi), 0x19)?;
    code.mov(dword_ptr(esi + 0x4), 400_001)?;
    code.mov(dword_ptr(esi + 0x8), POLE_NIGHT_PTR)?;
    code.add(dword_ptr(esp + 0x4), 0xC)?;
    code.add(dword_ptr(esp + 0x8), 1)?;
    code.add(esi, 0xC)?;
    code.inc(ecx)?;
    code.inc(edi)?;
    code.set_label(&mut endif)?;
    code.call(0x41E840)?;
    code.jmp(0x416F7F)?;
    inject(0x416F7A, code);

    // Inject drawing function for obstruction pole (Board::DrawGameObjects)
    let mut code = CodeAssembler::new(32)?;
    let mut not_draw_pole = code.create_label();
    code.mov(eax, dword_ptr(ebx - 0x8))?;
    code.cmp(eax, 0x19)?;
    code.jne(not_draw_pole)?;
    code.pushad()?;
    code.mov(ebx, dword_ptr(ebx))?;
    code.push(0)?;
    code.push(dword_ptr(ebx + 0x4))?;
    code.mov(eax, dword_ptr(ebp + 0xC))?;
    code.mov(ebx, dword_ptr(ebx))?;
    code.call(0x587150)?;
    code.popad()?;
    code.set_label(&mut not_draw_pole)?;
    code.cmp(eax, 0x18)?;
    code.jmp(0x416FA6)?;
    inject(0x416FA0, code);

    Ok(())
}
