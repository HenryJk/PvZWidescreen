use std::error::Error;

use iced_x86::code_asm::*;

use crate::{memory::inject, PAD};

pub unsafe fn patch_cursorobject() -> Result<(), Box<dyn Error>> {
    // Cursor Object placed PAD to the right of cursor (CursorObject::Update)
    let mut code = CodeAssembler::new(32)?;
    code.sub(ecx, 25 + PAD as i32)?;
    code.mov(dword_ptr(esi + 0x8), ecx)?;
    code.jmp(0x438805)?;
    inject(0x4387FF, code);

    // Move cursor preview PAD to the right (CursorPreview::Update)
    let mut code = CodeAssembler::new(32)?;
    code.sub(ebx, PAD as i32)?;
    code.push(ebx)?;
    code.mov(eax, ebp)?;
    code.mov(ecx, esi)?;
    code.jmp(0x438DEB)?;
    inject(0x438DE6, code);

    Ok(())
}
