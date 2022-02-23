use core::intrinsics::transmute;

use std::error::Error;

use iced_x86::code_asm::*;
use winapi::um::winnt::PAGE_READWRITE;

use crate::{
    memory::{alloc_mem, inject, patch},
    PAD, POLE_NIGHT_PTR, POLE_OFFSET, POLE_PTR, SLOT_MACHINE_OFFSET_PTR,
};

pub unsafe fn patch_board() -> Result<(), Box<dyn Error>> {
    // Draw image of level background with -220 - PAD offset (Board::DrawBackdrop)
    patch(0x416356, &transmute::<i16, [u8; 2]>(-220 - PAD));
    patch(0x41640D, &transmute::<i16, [u8; 2]>(-220 - PAD));
    patch(0x41648E, &transmute::<i16, [u8; 2]>(-220 - PAD));
    patch(0x4164B4, &transmute::<i16, [u8; 2]>(-220 - PAD));

    // Move crop area for level 1-4 sod unroll by PAD
    patch(0x416461, &transmute::<i16, [u8; 2]>(232 + PAD));

    // Increase start of level cutscene shift by PAD (CutScene::AnimateBoard)
    let mut code = CodeAssembler::new(32)?;
    code.add(esi, PAD as u32)?;
    code.mov(edi, dword_ptr(esp + 0x24))?;
    code.cmp(dword_ptr(ebp + 0x8), edi)?;
    code.jmp(0x43B8EC)?;
    inject(0x43B8E5, code);

    // Move obstruction pole (CutScene::AnimateBoard)
    let mut code = CodeAssembler::new(32)?;
    code.push(0)?;
    code.push(esi)?;
    code.add(esi, esi)?;
    code.add(esi, dword_ptr(esp))?;
    code.add(esi, (1600 - 2 * POLE_OFFSET - 3 * PAD) as i32)?;
    code.sar(esi, 1)?;
    code.mov(dword_ptr(POLE_PTR + 0x4), esi)?;
    code.mov(dword_ptr(POLE_NIGHT_PTR + 0x4), esi)?;
    code.mov(esi, dword_ptr(esp))?;
    code.call(eax)?;
    code.jmp(0x43B8FE)?;
    inject(0x43B8F9, code);

    // Change right edge of Board from 1180 to 1180 + PAD (CutScene::AnimateBoard)
    patch(0x43B916, &transmute::<i16, [u8; 2]>(1180 + PAD));
    patch(0x43BA53, &transmute::<i16, [u8; 2]>(1180 + PAD));

    // Move obstruction pole during camera paning (part 1) (CutScene::AnimateBoard)
    let mut code = CodeAssembler::new(32)?;
    code.add(eax, eax)?;
    code.add(eax, dword_ptr(esp))?;
    code.add(eax, (1600 - 2 * POLE_OFFSET - 3 * PAD) as i32)?;
    code.sar(eax, 1)?;
    code.mov(dword_ptr(POLE_PTR + 0x4), eax)?;
    code.mov(dword_ptr(POLE_NIGHT_PTR + 0x4), eax)?;
    code.mov(eax, dword_ptr(edi + 0xa4))?;
    code.jmp(0x43B93F)?;
    inject(0x43B939, code);

    // Change SeedChooserScreen offset to PAD (CutScene::AnimateBoard)
    let mut code = CodeAssembler::new(32)?;
    code.push(PAD as i32)?;
    code.mov(ecx, edi)?;
    code.call(eax)?;
    code.jmp(0x43B9A2)?;
    inject(0x43B99C, code);

    // Move obstruction pole during camera paning (part 2) (CutScene::AnimateBoard)
    let mut code = CodeAssembler::new(32)?;
    code.push(eax)?;
    code.add(eax, eax)?;
    code.add(eax, dword_ptr(esp))?;
    code.add(eax, (1600 - 2 * POLE_OFFSET - 3 * PAD) as i32)?;
    code.sar(eax, 1)?;
    code.mov(dword_ptr(POLE_PTR + 0x4), eax)?;
    code.mov(dword_ptr(POLE_NIGHT_PTR + 0x4), eax)?;
    code.mov(ecx, esi)?;
    code.call(edx)?;
    code.jmp(0x43BA79)?;
    inject(0x43BA74, code);

    // Board starts with offset PAD (LawnApp::MakeNewBoard)
    let mut code = CodeAssembler::new(32)?;
    code.mov(dword_ptr(POLE_PTR + 0x4), 800 - POLE_OFFSET as i32)?;
    code.mov(dword_ptr(POLE_NIGHT_PTR + 0x4), 800 - POLE_OFFSET as i32)?;
    code.mov(dword_ptr(SLOT_MACHINE_OFFSET_PTR), 2 * PAD as i32)?;
    code.push(0)?;
    code.push(PAD as i32)?;
    code.mov(ecx, eax)?;
    code.jmp(0x44F661)?;
    inject(0x44F65B, code);

    // Start level intro at 220 + PAD (CutScene::StartLevelIntro)
    let mut code = CodeAssembler::new(32)?;
    code.push(220 + PAD as i32)?;
    code.mov(dword_ptr(POLE_PTR + 0x4), 1130 - POLE_OFFSET as i32)?;
    code.mov(dword_ptr(POLE_NIGHT_PTR + 0x4), 1130 - POLE_OFFSET as i32)?;
    code.mov(dword_ptr(SLOT_MACHINE_OFFSET_PTR), PAD as i32)?;
    code.jmp(0x43B016)?;
    inject(0x43B011, code);

    // Set SEED_BANK_OFFSET_X to PAD
    patch(0x6A9EAC, &transmute::<i16, [u8; 2]>(PAD));

    // SeedChooserScreen offset changed to PAD (CutScene::AnimateBoard)
    let mut code = CodeAssembler::new(32)?;
    code.push(eax)?;
    code.push(PAD as i32)?;
    code.mov(ecx, esi)?;
    code.jmp(0x43BA26)?;
    inject(0x43BA21, code);

    // SeedBank final offset_x changed to 10 + PAD (CutScene::AnimateBoard)
    let mut code = CodeAssembler::new(32)?;
    code.push(4)?;
    code.push(10 + PAD as i32)?;
    code.push(eax)?;
    code.jmp(0x43BB5F)?;
    inject(0x43BB5A, code);

    // Set SeedBank->mX to 10 on Board::StartLevel (Board::StartLevel)
    let mut code = CodeAssembler::new(32)?;
    code.push(eax)?;
    code.mov(eax, dword_ptr(eax + 0x144))?;
    code.mov(dword_ptr(eax + 0x8), 10)?;
    code.pop(eax)?;
    code.sub(esp, 0x1C)?;
    code.push(edi)?;
    code.mov(edi, eax)?;
    code.mov(dword_ptr(SLOT_MACHINE_OFFSET_PTR), 2 * PAD as i32)?;
    code.jmp(0x40BE06)?;
    inject(0x40BE00, code);

    // SeedHitTest parameter decreased by PAD (SeedChooserScreen::UpdateCursor)
    let mut code = CodeAssembler::new(32)?;
    code.sub(dword_ptr(esp), PAD as i32)?;
    code.call(0x485D80)?;
    code.jmp(0x4850B1)?;
    inject(0x4850AC, code);

    // SeedHitTest parameter decreased by PAD (SeedChooserScreen::ShowToolTip)
    let mut code = CodeAssembler::new(32)?;
    code.sub(dword_ptr(esp), PAD as i32)?;
    code.call(0x485D80)?;
    code.jmp(0x4862FD)?;
    inject(0x4862F8, code);

    // DrawSeedPacket parameter increased by PAD for SEED_IN_BANK state (SeedChooserScreen::Draw)
    let mut code = CodeAssembler::new(32)?;
    code.add(eax, PAD as i32)?;
    code.sub(eax, dword_ptr(edx + 0x30))?;
    code.sub(ecx, dword_ptr(edx + 0x34))?;
    code.jmp(0x484A8B)?;
    inject(0x484A85, code);

    // Set Board->mX to PAD after shake (Board::Update)
    let mut code = CodeAssembler::new(32)?;
    code.mov(dword_ptr(ebp + 0x30), PAD as i32)?;
    code.mov(dword_ptr(ebp + 0x34), ebx)?;
    code.jmp(0x415EE7)?;
    inject(0x415EE1, code);

    // Increase Board->mX by PAD during shake (Board::Update)
    let mut code = CodeAssembler::new(32)?;
    code.add(eax, PAD as i32)?;
    code.xor(ecx, ecx)?;
    code.mov(dword_ptr(ebp + 0x30), eax)?;
    code.jmp(0x415F2F)?;
    inject(0x415F2A, code);

    // min_x -= PAD, max_x += PAD during view_lawn (start) (SeedChooserScreen::UpdateViewLawn)
    let mut code = CodeAssembler::new(32)?;
    code.add(dword_ptr(esp), PAD as i32)?;
    code.sub(dword_ptr(esp + 0x4), PAD as i32)?;
    code.call(0x511C40)?;
    code.jmp(0x484E4F)?;
    inject(0x484E4A, code);

    // Update Obstruction Pole during view_lawn (start) (SeedChooserScreen::UpdateViewLawn)
    let mut code = CodeAssembler::new(32)?;
    code.push(eax)?;
    code.add(eax, eax)?;
    code.add(eax, dword_ptr(esp))?;
    code.add(eax, (1600 - 2 * POLE_OFFSET - 3 * PAD) as i32)?;
    code.sar(eax, 1)?;
    code.mov(dword_ptr(POLE_PTR + 0x4), eax)?;
    code.mov(dword_ptr(POLE_NIGHT_PTR + 0x4), eax)?;
    code.mov(eax, dword_ptr(esp))?;
    code.mov(ecx, esi)?;
    code.call(edx)?;
    code.jmp(0x484E5F)?;
    inject(0x484E5A, code);

    // min_x -= PAD, max_x += PAD during view_lawn (return) (SeedChooserScreen::UpdateViewLawn)
    let mut code = CodeAssembler::new(32)?;
    code.sub(dword_ptr(esp), PAD as i32)?;
    code.add(dword_ptr(esp + 0x4), PAD as i32)?;
    code.call(0x511C40)?;
    code.jmp(0x484F3B)?;
    inject(0x484F36, code);

    // Update Obstruction Pole during view_lawn (return) (SeedChooserScreen::UpdateViewLawn)
    let mut code = CodeAssembler::new(32)?;
    code.push(eax)?;
    code.add(eax, eax)?;
    code.add(eax, dword_ptr(esp))?;
    code.add(eax, (1600 - 2 * POLE_OFFSET - 3 * PAD) as i32)?;
    code.sar(eax, 1)?;
    code.mov(dword_ptr(POLE_PTR + 0x4), eax)?;
    code.mov(dword_ptr(POLE_NIGHT_PTR + 0x4), eax)?;
    code.mov(eax, dword_ptr(esp))?;
    code.mov(ecx, esi)?;
    code.call(edx)?;
    code.jmp(0x484F4B)?;
    inject(0x484F46, code);

    // Board->mX = PAD, SeedChooserScreen->mX = PAD during view_lawn (view) (SeedChooserScreen::UpdateViewLawn)
    let mut code = CodeAssembler::new(32)?;
    code.mov(edi, PAD as i32)?;
    code.push(edi)?;
    code.call(edx)?;
    code.mov(eax, dword_ptr(ebp))?;
    code.jmp(0x484ECA)?;
    inject(0x484EC4, code);

    // SeedChooserScreen->mX = PAD during view_lawn (start) (SeedChooserScreen::UpdateViewLawn)
    let mut code = CodeAssembler::new(32)?;
    code.push(PAD as i32)?;
    code.mov(ecx, ebp)?;
    code.call(edx)?;
    code.mov(dword_ptr(esp), 0)?;
    code.jmp(0x484E99)?;
    inject(0x484E94, code);

    // SeedChooserScreen->mX = PAD during view_lawn (return) (SeedChooserScreen::UpdateViewLawn)
    let mut code = CodeAssembler::new(32)?;
    code.push(PAD as i32)?;
    code.mov(ecx, ebp)?;
    code.call(edx)?;
    code.mov(dword_ptr(esp), 0)?;
    code.jmp(0x484F86)?;
    inject(0x484F81, code);

    // Make x = -133 still get Board (Sexy::WidgetContainer::GetWidgetAtHelper)
    let mut code = CodeAssembler::new(32)?;
    let mut endif = code.create_label();
    code.push(eax)?;
    code.mov(ecx, esi)?;
    code.call(edx)?;
    code.sub(esp, 0x100)?;
    code.push(ecx)?;
    code.mov(ecx, dword_ptr(0x6A9EC0))?;
    code.mov(ecx, dword_ptr(ecx + 0x768))?;
    code.cmp(dword_ptr(esp), ecx)?;
    code.jne(endif)?;
    code.mov(dword_ptr(eax), 0)?;
    code.set_label(&mut endif)?;
    code.pop(ecx)?;
    code.add(esp, 0x100)?;
    code.jmp(0x5373A0)?;
    inject(0x53739B, code);

    // Make LawnMessage drawn centered (MessageWidget::Draw)
    patch(0x459C96, &transmute::<i16, [u8; 2]>(800 + 2 * PAD));
    patch(0x459CB3, &transmute::<i32, [u8; 4]>(-PAD as i32));
    patch(0x459E4B, &transmute::<i16, [u8; 2]>(800 + 2 * PAD));

    // Move [FLAG_COMPLETED] message draw placement by PAD
    patch(0x45A011, &transmute::<i16, [u8; 2]>(400 + PAD));

    // Make fadeout full screen
    let mut code = CodeAssembler::new(32)?;
    code.mov(dword_ptr(esp), -PAD as i32)?;
    code.mov(dword_ptr(esp + 0x8), 800 + 2 * PAD as i32)?;
    code.call(0x586D50)?;
    code.jmp(0x419F58)?;
    inject(0x419F53, code);

    // Move [HUGE_WAVE] message draw position by PAD (DrawSeedPacket)
    let mut code = CodeAssembler::new(32)?;
    let storage_ptr = alloc_mem(4, PAGE_READWRITE) as u32;
    patch(storage_ptr, &transmute::<i16, [u8; 2]>(PAD));
    code.fld(dword_ptr(eax + 0x8))?;
    code.fiadd(dword_ptr(storage_ptr))?;
    code.fstp(dword_ptr(eax + 0x8))?;
    code.call(0x511E50)?;
    code.jmp(0x45993D)?;
    inject(0x459938, code);

    // Move slotmachine reamin by PAD (Challenge::DrawSlotMachine)
    let mut code = CodeAssembler::new(32)?;
    code.fld(dword_ptr(esp + 0x34))?;
    code.fiadd(dword_ptr(SLOT_MACHINE_OFFSET_PTR))?;
    code.fstp(dword_ptr(esp + 0x34))?;
    code.call(0x472E40)?;
    code.jmp(0x4252D6)?;
    inject(0x4252D1, code);

    // Change losing sequence offset (CutScene::UpdateZombiesWon)
    let mut code = CodeAssembler::new(32)?;
    code.mov(dword_ptr(esp), PAD as i32)?;
    code.mov(dword_ptr(esp + 0x4), 220 + PAD as i32)?;
    code.call(0x511C40)?;
    code.jmp(0x43C463)?;
    inject(0x43C45E, code);

    // Move Zombie pre-game showcase by PAD + 30 (CutScene::PlaceAZombie)
    const ZOMBIE_OFFSET: i32 = PAD as i32 + 30;
    patch(0x43919B, &transmute::<i32, [u8; 4]>(830 + ZOMBIE_OFFSET));
    let storage_ptr = alloc_mem(4, PAGE_READWRITE) as u32;
    patch(storage_ptr, &transmute::<i32, [u8; 4]>(5 + ZOMBIE_OFFSET));
    let mut code = CodeAssembler::new(32)?;
    code.fisub(dword_ptr(storage_ptr))?;
    patch(0x43924B, &code.assemble(0)?);

    // Move detection area of zombie clicked by PAD (SeedChooserScreen::MouseDown)
    let mut code = CodeAssembler::new(32)?;
    code.add(dword_ptr(esp + 0x4), PAD as i32)?;
    code.call(0x40E780)?;
    code.jmp(0x486AFA)?;
    inject(0x486AF5, code);

    // (SeedChooserScreen::ShowToolTip)
    patch(0x486582, &transmute::<i16, [u8; 2]>(800 + 2 * PAD));
    patch(0x486588, &[0x90; 8]);

    // (ToolTipWidget::Draw)
    patch(0x51AAEA, &transmute::<i16, [u8; 2]>(800 + 2 * PAD));
    let width_ptr = alloc_mem(8, PAGE_READWRITE) as u32;
    patch(
        width_ptr,
        &transmute::<f64, [u8; 8]>(800.0 + 2.0 * PAD as f64),
    );
    patch(0x51AAD4, &transmute::<u32, [u8; 4]>(width_ptr));

    // Disable placing zombie at non-roof level too close to bush (CutScene::CanZombieGoInGridSpot)
    patch(0x439426, &[0x04, 0x72]);
    patch(0x439429, &[0x90; 5]);

    // Disable placing gargantuar at col = 1
    let mut code = CodeAssembler::new(32)?;
    code.cmp(esi, 0x1)?;
    code.jbe(0x43940D)?;
    code.test(ebx, ebx)?;
    code.jmp(0x4393F0)?;
    inject(0x4393EA, code);

    // Move Bobsled on SeedChooserScreen PAD to the right (CutScene::PlaceAZombie)
    patch(0x67A158, &transmute::<f32, [u8; 4]>(1105.0 + PAD as f32));

    // Move Bungee in SeedChooserScreen to top right corner (CutScene::PlaceAZombie)
    const BUNGEE_X: f32 = 1250.0;
    let mut code = CodeAssembler::new(32)?;
    let mut on_roof = code.create_label();
    let mut endif = code.create_label();
    code.push(esi)?;
    code.mov(esi, dword_ptr(esi + 0x4))?;
    code.mov(esi, dword_ptr(esi + 0x554C))?;
    code.cmp(esi, 3)?;
    code.ja(on_roof)?;
    code.pop(esi)?;
    code.mov(dword_ptr(esi + 0x2C), transmute::<f32, i32>(BUNGEE_X))?;
    code.jmp(endif)?;
    code.set_label(&mut on_roof)?;
    code.pop(esi)?;
    code.set_label(&mut endif)?;
    code.fld(dword_ptr(0x679800))?;
    code.jmp(0x43938E)?;
    inject(0x439388, code);

    // Center GameOver Dialog (GameOverDialog::GameOverDialog)
    patch(0x457D76, &transmute::<i16, [u8; 2]>(800 + 2 * PAD));
    patch(0x457E01, &transmute::<i16, [u8; 2]>(635 + PAD));
    patch(0x457FEC, &transmute::<i16, [u8; 2]>(635 + PAD));

    // Make winning fullscreen flash 800 + 2 * PAD wide (RenderParticle)
    patch(0x51813E, &transmute::<i16, [u8; 2]>(800 + 2 * PAD));

    Ok(())
}
