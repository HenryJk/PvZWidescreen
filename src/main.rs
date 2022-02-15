mod memory;
mod patch_almanac;
mod patch_bush;
mod patch_challenge;
mod patch_store;

use iced_x86::code_asm::*;
use memory::{alloc_mem, patch};
use patch_almanac::patch_almanac;
use patch_bush::patch_bush;
use patch_challenge::patch_challenge;
use patch_store::patch_store;

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

const OLD_WIDTH_ADDRESS: [u32; 10] = [
    0x4011F8, 0x407672, 0x4415DA, 0x441908, 0x44193E, 0x44EC12, 0x450137, 0x45047D, 0x486A2C,
    0x51813E,
];

const PAD: i16 = 133;
const POST_OFFSET: i16 = 27;

static mut H_PROCESS: *mut c_void = null_mut();
static mut POLE_PTR: u32 = 0;
static mut POLE_NIGHT_PTR: u32 = 0;

fn main() -> Result<(), Box<dyn Error>> {
    let pvz_process = Command::new("PlantsVsZombies.exe")
        .creation_flags(0x00000004)
        .spawn()
        .expect("PlantsVsZombies.exe not found");

    unsafe {
        H_PROCESS = pvz_process.as_raw_handle() as *mut c_void;
        POLE_PTR = alloc_mem(12, PAGE_READWRITE) as u32;
        patch(
            POLE_PTR + 0x4,
            &transmute::<i16, [u8; 2]>(800 - POST_OFFSET),
        );
        POLE_NIGHT_PTR = alloc_mem(12, PAGE_READWRITE) as u32;
        patch(
            POLE_NIGHT_PTR + 0x4,
            &transmute::<i16, [u8; 2]>(800 - POST_OFFSET),
        );

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
        patch(0x41648E, &transmute::<i16, [u8; 2]>(-220 - PAD));

        // Animate board of zen garden cutscene with -PAD offset (CutScene::AnimateBoard)
        let mut code = CodeAssembler::new(32)?;
        code.mov(dword_ptr(esp + 0x4), -PAD as i32)?;
        code.call(0x511C40)?;
        code.jmp(0x43BA69)?;
        inject(0x43BA64, code);

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
        code.add(esi, (1600 - 2 * POST_OFFSET - 3 * PAD) as i32)?;
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
        code.add(eax, (1600 - 2 * POST_OFFSET - 3 * PAD) as i32)?;
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
        code.add(eax, (1600 - 2 * POST_OFFSET - 3 * PAD) as i32)?;
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
        code.mov(dword_ptr(POLE_PTR + 0x4), 800 - POST_OFFSET as i32)?;
        code.mov(dword_ptr(POLE_NIGHT_PTR + 0x4), 800 - POST_OFFSET as i32)?;
        code.push(0)?;
        code.push(PAD as i32)?;
        code.mov(ecx, eax)?;
        code.jmp(0x44F661)?;
        inject(0x44F65B, code);

        // Start level intro at 220 + PAD (CutScene::StartLevelIntro)
        let mut code = CodeAssembler::new(32)?;
        code.push(220 + PAD as i32)?;
        code.mov(dword_ptr(POLE_PTR + 0x4), 1130 - POST_OFFSET as i32)?;
        code.mov(dword_ptr(POLE_NIGHT_PTR + 0x4), 1130 - POST_OFFSET as i32)?;
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
        code.add(eax, (1600 - 2 * POST_OFFSET - 3 * PAD) as i32)?;
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
        code.add(eax, (1600 - 2 * POST_OFFSET - 3 * PAD) as i32)?;
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

        patch_almanac()?;
        patch_bush()?;
        patch_challenge()?;
        patch_store()?;

        // std::thread::sleep(std::time::Duration::from_secs(10));
        NtResumeProcess(H_PROCESS);
    }
    Ok(())
}
