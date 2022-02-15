use std::error::Error;

use iced_x86::code_asm::*;

use crate::{memory::inject, PAD};

pub unsafe fn patch_store() -> Result<(), Box<dyn Error>> {
    // Move StoreScreen.mX to PAD (LawnApp::ShowStoreScreen)
    let mut code = CodeAssembler::new(32)?;
    code.call(0x489DA0)?;
    code.mov(dword_ptr(eax + 0x30), PAD as i32)?;
    code.jmp(0x44FD79)?;
    inject(0x44FD74, code);

    // Overhaul car drawing logic (StoreScreen::Draw)
    const IMAGE_STORE_BACKGROUND: u32 = 0x6A778C;
    const IMAGE_STORE_BACKGROUNDNIGHT: u32 = 0x6A7B28;
    const IMAGE_STORE_CAR: u32 = 0x6A763C;
    const IMAGE_STORE_CAR_NIGHT: u32 = 0x6A77FC;
    const IMAGE_STORE_CARCLOSED: u32 = 0x6A74A0;
    const IMAGE_STORE_CARCLOSED_NIGHT: u32 = 0x6A7560;
    const IMAGE_STORE_HATCHBACKOPEN: u32 = 0x6A73DC;
    const DRAWIMAGE_FN: u64 = 0x587150;

    let mut code = CodeAssembler::new(32)?;
    let mut day = code.create_label();
    let mut night = code.create_label();
    let mut endif = code.create_label();
    let mut car_closed = code.create_label();
    let mut exit = code.create_label();
    code.jz(day)?;
    code.mov(eax, dword_ptr(eax + 0x24))?;
    code.cmp(eax, 11)?;
    code.jl(day)?;
    code.cmp(eax, 21)?;
    code.jl(night)?;
    code.cmp(eax, 31)?;
    code.jl(day)?;
    code.cmp(eax, 41)?;
    code.jl(night)?;
    code.cmp(eax, 50)?;
    code.jnz(day)?;
    code.set_label(&mut night)?;
    code.push(true as i32)?;
    code.jmp(endif)?;
    code.set_label(&mut day)?;
    code.push(false as i32)?;
    code.set_label(&mut endif)?;
    code.mov(eax, dword_ptr(esi + 0x1A8))?;
    code.add(eax, 138)?;
    code.push(eax)?;
    code.mov(eax, dword_ptr(esi + 0x1A4))?;
    code.add(eax, 196)?;
    code.push(eax)?;
    code.mov(ebx, dword_ptr(IMAGE_STORE_BACKGROUND))?;
    code.cmp(byte_ptr(esp + 0x8), 0)?;
    code.cmovnz(ebx, dword_ptr(IMAGE_STORE_BACKGROUNDNIGHT))?;
    code.push(0)?;
    code.push(-PAD as i32)?;
    code.mov(eax, edi)?;
    code.call(DRAWIMAGE_FN)?;
    code.mov(ebx, dword_ptr(IMAGE_STORE_CAR))?;
    code.cmp(byte_ptr(esp + 0x8), 0)?;
    code.cmovnz(ebx, dword_ptr(IMAGE_STORE_CAR_NIGHT))?;
    code.push(dword_ptr(esp + 0x4))?;
    code.push(dword_ptr(esp + 0x4))?;
    code.mov(eax, edi)?;
    code.call(DRAWIMAGE_FN)?;
    code.cmp(dword_ptr(esi + 0x19C), 0)?;
    code.jnz(car_closed)?;
    code.cmp(byte_ptr(esi + 0x1A0), 0)?;
    code.jz(car_closed)?;
    code.mov(ebx, dword_ptr(IMAGE_STORE_HATCHBACKOPEN))?;
    code.mov(eax, dword_ptr(esi + 0x1A8))?;
    code.push(eax)?;
    code.mov(eax, dword_ptr(esi + 0x1A4))?;
    code.add(eax, 299)?;
    code.push(eax)?;
    code.mov(eax, edi)?;
    code.call(DRAWIMAGE_FN)?;
    code.jmp(exit)?;
    code.set_label(&mut car_closed)?;
    code.mov(ebx, dword_ptr(IMAGE_STORE_CARCLOSED))?;
    code.push(dword_ptr(esp + 0x4))?;
    code.push(dword_ptr(esp + 0x4))?;
    code.mov(eax, edi)?;
    code.call(DRAWIMAGE_FN)?;
    code.cmp(byte_ptr(esp + 0x8), 0)?;
    code.jz(exit)?;
    code.mov(ebx, dword_ptr(IMAGE_STORE_CARCLOSED_NIGHT))?;
    code.push(dword_ptr(esp + 0x4))?;
    code.push(dword_ptr(esp + 0x4))?;
    code.mov(eax, edi)?;
    code.call(DRAWIMAGE_FN)?;
    code.set_label(&mut exit)?;
    code.add(esp, 0xC)?;
    code.jmp(0x48B6B6)?;
    inject(0x48B529, code);

    Ok(())
}
