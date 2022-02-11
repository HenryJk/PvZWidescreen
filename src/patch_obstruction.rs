use std::error::Error;

use iced_x86::code_asm::*;
use winapi::um::winnt::PAGE_READWRITE;

use crate::memory::{alloc_mem, inject, patch};

pub unsafe fn patch_obstruction() -> Result<(), Box<dyn Error>> {
    // Load IMAGE_BACKGROUND5_OBSTRUCTION_POLE (Sexy::ExtractDelayLoad_Background5Resources)
    let image_background5_obstruction_pole_ptr = alloc_mem(4, PAGE_READWRITE) as u32;
    let storage_ptr = alloc_mem(0x100, PAGE_READWRITE) as u32;
    const OBSTRUCTION_POLE_NAME: &str = "IMAGE_BACKGROUND5_OBSTRUCTION_POLE";
    patch(storage_ptr + 0x20, OBSTRUCTION_POLE_NAME.as_bytes());
    let mut code = CodeAssembler::new(32)?;
    code.push(OBSTRUCTION_POLE_NAME.len() as u32)?;
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
    code.mov(dword_ptr(image_background5_obstruction_pole_ptr), eax)?;
    code.call(0x59A8D0)?;
    code.mov(eax, 1)?;
    code.mov(ecx, dword_ptr(ebp - 0xC))?;
    code.jmp(0x475925)?;
    inject(0x475920, code);

    // Put Obstruction pole on draw queue (Board::DrawGameObjects)
    let mut code = CodeAssembler::new(32)?;
    let mut branch_not_taken = code.create_label();
    let mut branch_end = code.create_label();
    code.mov(esi, dword_ptr(esp + 0x14))?;
    code.cmp(dword_ptr(esi + 0x554C), 4)?;
    code.jne(branch_not_taken)?;
    code.mov(ecx, dword_ptr(esi + 0x30))?;
    code.mov(esi, dword_ptr(esp + 0x4))?;
    code.shl(ecx, 1)?;
    code.add(ecx, 407)?;
    code.mov(dword_ptr(esi), 0x19)?;
    code.mov(dword_ptr(esi + 0x4), 399_999)?;
    code.mov(dword_ptr(esi + 0x8), ecx)?;
    code.add(dword_ptr(esp + 0x4), 0xC)?;
    code.add(dword_ptr(esp + 0x8), 1)?;
    code.add(esi, 0xC)?;
    code.mov(ecx, dword_ptr(esp + 0x8))?;
    code.inc(ecx)?;
    code.inc(edi)?;
    code.jmp(branch_end)?;
    code.set_label(&mut branch_not_taken)?;
    code.mov(esi, dword_ptr(esp + 0x4))?;
    code.set_label(&mut branch_end)?;
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
    code.push(0)?;
    code.push(dword_ptr(ebx))?;
    code.mov(eax, dword_ptr(ebp + 0xC))?;
    code.mov(ebx, dword_ptr(image_background5_obstruction_pole_ptr))?;
    code.call(0x587150)?;
    code.popad()?;
    code.set_label(&mut not_draw_pole)?;
    code.cmp(eax, 0x18)?;
    code.jmp(0x416FA6)?;
    inject(0x416FA0, code);

    Ok(())
}
