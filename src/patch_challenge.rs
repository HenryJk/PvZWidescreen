use std::error::Error;

use iced_x86::code_asm::*;

use crate::{memory::inject, PAD};

pub unsafe fn patch_challenge() -> Result<(), Box<dyn Error>> {
    // Draw IMAGE_CHALLENGE_BACKGROUND at -PAD offset (ChallengeScreen::Draw)
    let mut code = CodeAssembler::new(32)?;
    code.mov(dword_ptr(esp), -PAD as i32)?;
    code.call(0x587150)?;
    code.jmp(0x42F1A5)?;
    inject(0x42F1A0, code);

    // Move ChallengeScreen to PAD at creation (LawnApp::ShowChallengeScreen)
    let mut code = CodeAssembler::new(32)?;
    code.push(0)?;
    code.push(PAD as i32)?;
    code.mov(ecx, eax)?;
    code.jmp(0x44FCA6)?;
    inject(0x44FCA0, code);

    Ok(())
}
