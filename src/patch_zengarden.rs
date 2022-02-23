use core::intrinsics::transmute;

use std::error::Error;

use iced_x86::code_asm::*;

use crate::{
    memory::{inject, patch},
    PAD,
};

pub unsafe fn patch_zengarden() -> Result<(), Box<dyn Error>> {
    // Draw waveside on the left side of aquarium and wavecenter at 640 (Board::DrawUiBottom)
    let mut code = CodeAssembler::new(32)?;
    code.call(0x586CF0)?;
    code.mov(ecx, dword_ptr(0x6A724C))?;
    code.push(40)?;
    code.push(-PAD as i32)?;
    code.push(edi)?;
    code.mov(eax, esi)?;
    code.call(0x587E50)?;
    code.mov(ecx, dword_ptr(0x6A7AD0))?;
    code.push(40)?;
    code.push(640)?;
    code.push(edi)?;
    code.mov(eax, esi)?;
    code.call(0x587E50)?;
    code.jmp(0x41A0F6)?;
    inject(0x41A0F1, code);

    // Change left waveside to wavecenter (Board::DrawUiBottom)
    patch(0x41A0F8, &[0xD0, 0x7A, 0x6A, 0x00]);

    // Move right waveside to 800 + PAD (Board::DrawUiBottom)
    patch(0x41A162, &transmute::<i16, [u8; 2]>(800 + PAD));

    // Animate board of zen garden cutscene with -PAD offset (CutScene::AnimateBoard)
    let mut code = CodeAssembler::new(32)?;
    code.mov(dword_ptr(esp + 0x4), -PAD as i32)?;
    code.call(0x511C40)?;
    code.jmp(0x43BA69)?;
    inject(0x43BA64, code);

    // Draw image of zen garden background with -PAD offset (Board::DrawBackdrop)
    let mut code = CodeAssembler::new(32)?;
    code.mov(dword_ptr(esp), -PAD as i32)?;
    code.call(0x587150)?;
    code.jmp(0x4164A9)?;
    inject(0x4164A4, code);

    Ok(())
}
