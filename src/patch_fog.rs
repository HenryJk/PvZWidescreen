use std::error::Error;

use iced_x86::code_asm::*;

use crate::memory::{inject, patch};

pub unsafe fn patch_fog() -> Result<(), Box<dyn Error>> {
    // Move Fog Column 2 to the left (Board::LeftFogColumn)
    patch(0x41C1D2, &[0x04]);
    patch(0x41C1DC, &[0x03]);
    patch(0x41C201, &[0x02]);

    // Move inlined Board::LeftFogColumn result 2 to the left (Board::ClearFogAroundPlant)
    let mut code = CodeAssembler::new(32)?;
    code.sub(dword_ptr(esp), 2)?;
    code.push(ebp)?;
    code.push(esi)?;
    code.mov(esi, dword_ptr(esp + 0x20))?;
    code.jmp(0x41A4C4)?;
    inject(0x41A4BE, code);

    // Plant->mCol reduced by 2 for the purpose of fog logic (1st part) (Board::ClearFogAroundPlant)
    let mut code = CodeAssembler::new(32)?;
    code.mov(ebp, eax)?;
    code.sub(ebp, ebx)?;
    code.add(eax, ebx)?;
    code.sub(eax, 2)?;
    code.sub(ebp, 2)?;
    code.jmp(0x41A4CD)?;
    inject(0x41A4C7, code);

    // Plant->mCol reduced by 2 for the purpose of fog logic (2nd part) (Board::ClearFogAroundPlant)
    let mut code = CodeAssembler::new(32)?;
    code.mov(eax, dword_ptr(esp + 0x10))?;
    code.sub(eax, dword_ptr(esi + 0x28))?;
    code.add(eax, 2)?;
    code.jmp(0x41A53A)?;
    inject(0x41A533, code);

    // Draw fog offset 160 to the right (Board::DrawFog)
    let mut code = CodeAssembler::new(32)?;
    code.call(0x6397D0)?;
    code.add(eax, 160)?;
    code.jmp(0x41A976)?;
    inject(0x41A971, code);

    Ok(())
}
