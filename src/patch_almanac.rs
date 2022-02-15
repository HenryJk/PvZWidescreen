use std::error::Error;

use iced_x86::code_asm::*;

use crate::{memory::inject, PAD};

pub unsafe fn patch_almanac() -> Result<(), Box<dyn Error>> {
    // Move AlmanacDialog PAD to the right (AlmanacDialog::AlmanacDialog)
    let mut code = CodeAssembler::new(32)?;
    code.mov(dword_ptr(esp), PAD as i32)?;
    code.call(0x4573D0)?;
    code.jmp(0x401214)?;
    inject(0x40120F, code);

    // Reduce SeedHitTest parameter by PAD (AlmanacDialog::Update)
    let mut code = CodeAssembler::new(32)?;
    code.sub(dword_ptr(esp + 0x4), PAD as i32)?;
    code.sub(edi, PAD as i32)?;
    code.call(0x403940)?;
    code.jmp(0x401DC1)?;
    inject(0x401DBC, code);

    // Draw IMAGE_ALMANAC_INDEXBACK at -PAD offset (AlmanacDialog::DrawIndex)
    let mut code = CodeAssembler::new(32)?;
    code.mov(dword_ptr(esp), -PAD as i32)?;
    code.call(0x587150)?;
    code.jmp(0x401EA7)?;
    inject(0x401EA2, code);

    // Draw IMAGE_ALMANAC_PLANTBACK at -PAD offset (AlmanacDialog::DrawPlants)
    let mut code = CodeAssembler::new(32)?;
    code.mov(dword_ptr(esp), -PAD as i32)?;
    code.call(0x587150)?;
    code.jmp(0x402097)?;
    inject(0x402092, code);

    // Reduce SeedHitTest parameter by PAD (AlmanacDialog::DrawPlants)
    let mut code = CodeAssembler::new(32)?;
    code.sub(dword_ptr(esp + 0x4), PAD as i32)?;
    code.call(0x403940)?;
    code.jmp(0x40219A)?;
    inject(0x402195, code);

    // Draw IMAGE_ALMANAC_ZOMBIEBACK at -PAD offset (AlmanacDialog::DrawZombies)
    let mut code = CodeAssembler::new(32)?;
    code.mov(dword_ptr(esp), -PAD as i32)?;
    code.call(0x587150)?;
    code.jmp(0x402C37)?;
    inject(0x402C32, code);

    // Reduce ZombieHitTest parameter by PAD (AlmanacDialog::Update)
    let mut code = CodeAssembler::new(32)?;
    code.sub(edi, PAD as i32)?;
    code.call(0x403BB0)?;
    code.jmp(0x402D58)?;
    inject(0x402D53, code);

    // Change graphics.mCliptRect.mX = -PAD and mWidth = 800 + 2 * PAD during draw (AlmanacDialog::Draw)
    let mut code = CodeAssembler::new(32)?;
    code.mov(ebp, dword_ptr(esp + 0x8))?;
    code.mov(dword_ptr(ebp + 0x20), 0)?;
    code.mov(dword_ptr(ebp + 0x28), 800 + 2 * PAD as i32)?;
    code.mov(ebp, esp)?;
    code.and(esp, -8)?;
    code.jmp(0x403816)?;
    inject(0x403811, code);

    Ok(())
}
