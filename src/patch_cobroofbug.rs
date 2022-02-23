use std::error::Error;

use iced_x86::code_asm::*;

use crate::memory::inject;

pub unsafe fn patch_cobroofbug() -> Result<(), Box<dyn Error>> {
    let mut code = CodeAssembler::new(32)?;
    let mut not_cob = code.create_label();
    code.fstp(dword_ptr(esp + 0x8))?;
    code.cmp(dword_ptr(esi + 0x5C), 11)?;
    code.jne(not_cob)?;
    code.mov(dword_ptr(esp + 0x8), 0)?;
    code.set_label(&mut not_cob)?;
    code.fld(dword_ptr(esp + 0x8))?;
    code.cmp(eax, 8)?;
    code.jmp(0x46DCE3)?;
    inject(0x46DCDB, code);

    Ok(())
}
