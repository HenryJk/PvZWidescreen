mod memory;
mod patch_almanac;
mod patch_awardscreen;
mod patch_bush;
mod patch_challenge;
mod patch_cursorobject;
mod patch_dialogs;
mod patch_fog;
mod patch_gameselector;
mod patch_intro;
mod patch_store;
mod patch_titlescreen;
mod patch_zengarden;

use iced_x86::code_asm::*;
use memory::{alloc_mem, patch};
use patch_almanac::patch_almanac;
use patch_awardscreen::patch_awardscreen;
use patch_bush::patch_bush;
use patch_challenge::patch_challenge;
use patch_cursorobject::patch_cursorobject;
use patch_dialogs::patch_dialogs;
use patch_fog::patch_fog;
use patch_gameselector::patch_gameselector;
use patch_intro::patch_intro;
use patch_store::patch_store;
use patch_titlescreen::patch_titlescreen;
use patch_zengarden::patch_zengarden;

use core::intrinsics::transmute;

use std::{
    error::Error,
    os::windows::{prelude::AsRawHandle, process::CommandExt},
    process::Command,
    ptr::null_mut,
};

use ntapi::ntpsapi::NtResumeProcess;
use winapi::{ctypes::c_void, um::winnt::PAGE_READWRITE};

use memory::inject;

const OLD_WIDTH_ADDRESS: [u32; 2] = [0x44EC12, 0x51813E];

const PAD: i16 = 133;
const POLE_OFFSET: i16 = 27;

static mut H_PROCESS: *mut c_void = null_mut();
static mut POLE_PTR: u32 = 0;
static mut POLE_NIGHT_PTR: u32 = 0;
static mut SLOT_MACHINE_OFFSET_PTR: u32 = 0;
static mut PAD_CONST_PTR: u32 = 0;

fn main() -> Result<(), Box<dyn Error>> {
    let pvz_process = Command::new("PlantsVsZombies.exe")
        .creation_flags(0x00000004)
        .spawn()
        .expect("PlantsVsZombies.exe not found");

    unsafe {
        H_PROCESS = pvz_process.as_raw_handle() as *mut c_void;
        POLE_PTR = alloc_mem(12, PAGE_READWRITE) as u32;
        SLOT_MACHINE_OFFSET_PTR = alloc_mem(4, PAGE_READWRITE) as u32;
        PAD_CONST_PTR = alloc_mem(4, PAGE_READWRITE) as u32;
        patch(
            POLE_PTR + 0x4,
            &transmute::<i16, [u8; 2]>(800 - POLE_OFFSET),
        );
        POLE_NIGHT_PTR = alloc_mem(12, PAGE_READWRITE) as u32;
        patch(
            POLE_NIGHT_PTR + 0x4,
            &transmute::<i16, [u8; 2]>(800 - POLE_OFFSET),
        );
        patch(PAD_CONST_PTR, &transmute::<i32, [u8; 4]>(PAD as i32));

        for address in OLD_WIDTH_ADDRESS {
            patch(address, &transmute::<i16, [u8; 2]>(800 + 2 * PAD));
        }

        // Draw image of zen garden background with -PAD offset (Board::DrawBackdrop)
        let mut code = CodeAssembler::new(32)?;
        code.mov(dword_ptr(esp), -PAD as i32)?;
        code.call(0x587150)?;
        code.jmp(0x4164A9)?;
        inject(0x4164A4, code);

        // Draw image of level background with -220 - PAD offset (Board::DrawBackdrop)
        patch(0x416356, &transmute::<i16, [u8; 2]>(-220 - PAD));
        patch(0x41640D, &transmute::<i16, [u8; 2]>(-220 - PAD));
        patch(0x41648E, &transmute::<i16, [u8; 2]>(-220 - PAD));
        patch(0x4164B4, &transmute::<i16, [u8; 2]>(-220 - PAD));

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

        // Change right edge of Board from 1180 to 1180 + PAD (CutScene::AnimateBoard)
        patch(0x43BA53, &transmute::<i16, [u8; 2]>(1180 + PAD));

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

        patch_almanac()?;
        patch_awardscreen()?;
        patch_bush()?;
        patch_challenge()?;
        patch_cursorobject()?;
        patch_dialogs()?;
        patch_gameselector()?;
        patch_fog()?;
        patch_intro()?;
        patch_store()?;
        patch_titlescreen()?;
        patch_zengarden()?;

        // Make LawnMessage drawn centered
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

        patch(0x457D76, &transmute::<i16, [u8; 2]>(800 + 2 * PAD));
        patch(0x457E01, &transmute::<i16, [u8; 2]>(635 + PAD));
        patch(0x457FEC, &transmute::<i16, [u8; 2]>(635 + PAD));

        // (CutScene::PlaceAZombie)
        patch(0x43919B, &transmute::<i16, [u8; 2]>(830 + PAD));

        let mut code = CodeAssembler::new(32)?;
        code.add(dword_ptr(esp + 0x4), PAD as i32)?;
        code.call(0x40E780)?;
        code.jmp(0x486AFA)?;
        inject(0x486AF5, code);

        // (SeedChooserScreen::ShowToolTip)
        patch(0x486582, &transmute::<i16, [u8; 2]>(800 + 2 * PAD));

        // std::thread::sleep(std::time::Duration::from_secs(10));
        NtResumeProcess(H_PROCESS);
    }
    Ok(())
}
