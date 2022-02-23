use std::error::Error;

use iced_x86::code_asm::*;

use crate::{memory::inject, PAD};

pub unsafe fn patch_button() -> Result<(), Box<dyn Error>> {
    // Move menu button draw offset_x by PAD (SeedChooserScreen::Draw)
    let mut code = CodeAssembler::new(32)?;
    code.fld(dword_ptr(esp + 0x48))?;
    code.sub(esp, 0x100)?;
    code.push(dword_ptr(ebx + 0x30))?;
    code.sub(dword_ptr(esp), PAD as i32)?;
    code.fisub(dword_ptr(esp))?;
    code.add(esp, 0x104)?;
    code.jmp(0x484BE5)?;
    inject(0x484BDE, code);

    // Move GameButton PAD to the right (GameButton::Update)
    let mut code = CodeAssembler::new(32)?;
    code.mov(edx, dword_ptr(eax + 0xE0))?;
    code.sub(edx, PAD as i32)?;
    code.jmp(0x44833E)?;
    inject(0x448338, code);

    // Move GameButton that have parent PAD to the left (GameButton::Update)
    let mut code = CodeAssembler::new(32)?;
    code.add(edx, PAD as i32)?;
    code.sub(edx, dword_ptr(ebp + 0x30))?;
    code.sub(eax, dword_ptr(ebp + 0x34))?;
    code.jmp(0x448355)?;
    inject(0x44834F, code);

    Ok(())
}
