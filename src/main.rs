mod trampoline;

use trampoline::Trampoline;

use std::{
    mem::size_of_val,
    os::windows::{prelude::AsRawHandle, process::CommandExt},
    process::Command,
    ptr::null_mut,
};

use ntapi::ntpsapi::NtResumeProcess;
use winapi::{
    ctypes::{c_uchar, c_ushort, c_void},
    um::memoryapi::WriteProcessMemory,
};

const OLD_WIDTH_ADDRESS: [u32; 7] = [
    0x4011F8, 0x407672, 0x4415DA, 0x441908, 0x44193E, 0x44EC12, 0x51813E,
];
const NEW_WIDTH: u16 = 1066;

fn main() {
    let pvz_process = Command::new("PlantsVsZombies.exe")
        .creation_flags(0x00000004)
        .spawn()
        .expect("PlantsVsZombies.exe not found");
    let h_process = pvz_process.as_raw_handle() as *mut c_void;
    unsafe {
        let lp_buffer = &NEW_WIDTH as *const c_ushort as *const c_void;
        for address in OLD_WIDTH_ADDRESS {
            WriteProcessMemory(
                h_process,
                address as *mut c_void,
                lp_buffer,
                size_of_val(&NEW_WIDTH),
                null_mut(),
            );
        }

        // Draw image of zen garden background with -133 offset (Board::DrawBackdrop)
        Trampoline::new(h_process)
            .add_custom(&[0xc7, 0x04, 0x24, 0x7b, 0xff, 0xff, 0xff])
            .call(0x587150)
            .jump(0x4164A9)
            .inject(0x4164A4);

        // Draw image of level background with -353 offset (Board::DrawBackdrop)
        WriteProcessMemory(
            h_process,
            0x41648E as *mut c_void,
            &[0x9Fu8, 0xFEu8] as *const c_uchar as *const c_void,
            2,
            null_mut(),
        );

        // Animate board of zen garden cutscene with -133 offset (CutScene::AnimateBoard)
        Trampoline::new(h_process)
            .add_custom(&[0xC7, 0x44, 0x24, 0x04, 0x7B, 0xFF, 0xFF, 0xFF])
            .call(0x511C40)
            .jump(0x43BA69)
            .inject(0x43BA64);

        // Increase start of level cutscene shift by 133 (CutScene::AnimateBoard)
        Trampoline::new(h_process)
            .add_custom(&[0x81, 0xC6, 0x85, 0x00, 0x00, 0x00])
            .add_custom(&[0x8B, 0x7C, 0x24, 0x24])
            .add_custom(&[0x39, 0x7D, 0x08])
            .jump(0x43B8EC)
            .inject(0x43B8E5);

        // Change right edge of Board from 1180 to 1313 (CutScene::AnimateBoard)
        WriteProcessMemory(
            h_process,
            0x43B916 as *mut c_void,
            &[0x21u8, 0x05u8] as *const c_uchar as *const c_void,
            2,
            null_mut(),
        );

        // Change SeedChooserScreen offset to 133 (CutScene::AnimateBoard)
        Trampoline::new(h_process)
            .add_custom(&[0x68, 0x85, 0x00, 0x00, 0x00])
            .add_custom(&[0x8B, 0xCF])
            .add_custom(&[0xFF, 0xD0])
            .jump(0x43B9A2)
            .inject(0x43B99C);

        // Change right edge of Board from 1180 to 1313 (CutScene::AnimateBoard)
        WriteProcessMemory(
            h_process,
            0x43BA53 as *mut c_void,
            &[0x21u8, 0x05u8] as *const c_uchar as *const c_void,
            2,
            null_mut(),
        );

        // Disable SeedChooserScreen->mMenuButton->mX draw offset reduction (SeedChooserScreen::Draw)
        // WriteProcessMemory(
        //     h_process,
        //     0x484BE2 as *mut c_void,
        //     &[0x90u8, 0x90, 0x90] as *const c_uchar as *const c_void,
        //     3,
        //     null_mut(),
        // );
        Trampoline::new(h_process)
            .add_custom(&[0xD9, 0x44, 0x24, 0x48])
            .add_custom(&[0xFF, 0x73, 0x30])
            .add_custom(&[0x81, 0x2C, 0x24, 0x85, 0x00, 0x00, 0x00])
            .add_custom(&[0xDA, 0x24, 0x24])
            .add_custom(&[0x83, 0xC4, 0x04])
            .jump(0x484BE5)
            .inject(0x484BDE);

        // Board starts with offset 133 (LawnApp::MakeNewBoard)
        Trampoline::new(h_process)
            .add_custom(&[0x6A, 0x00])
            .add_custom(&[0x68, 0x85, 0x00, 0x00, 0x00])
            .add_custom(&[0x89, 0xC1])
            .jump(0x44F661)
            .inject(0x44F65B);

        // Start level board at 353 (???)
        WriteProcessMemory(
            h_process,
            0x43B012 as *mut c_void,
            &[0x61u8, 0x01u8] as *const c_uchar as *const c_void,
            2,
            null_mut(),
        );

        // Set SEED_BANK_OFFSET_X to 133
        WriteProcessMemory(
            h_process,
            0x6A9EAC as *mut c_void,
            &[0x85u8] as *const c_uchar as *const c_void,
            1,
            null_mut(),
        );

        // SeedChooserScreen offset changed to 133 (CutScene::AnimateBoard)
        Trampoline::new(h_process)
            .add_custom(&[0x50])
            .add_custom(&[0x68, 0x85, 0x00, 0x00, 0x00])
            .add_custom(&[0x89, 0xF1])
            .jump(0x43BA26)
            .inject(0x43BA21);

        // SeedBank final offset_x changed to 143 (CutScene::AnimateBoard)
        Trampoline::new(h_process)
            .add_custom(&[0x6A, 0x04])
            .add_custom(&[0x68, 0x8F, 0x00, 0x00, 0x00])
            .add_custom(&[0x50])
            .jump(0x43BB5F)
            .inject(0x43BB5A);

        // Set SeedBank->mX to 10 on Board::StartLevel (Board::StartLevel)
        Trampoline::new(h_process)
            .add_custom(&[0x50])
            .add_custom(&[0x8B, 0x80, 0x44, 0x01, 0x00, 0x00])
            .add_custom(&[0xC7, 0x40, 0x08, 0x0A, 0x00, 0x00, 0x00])
            .add_custom(&[0x58])
            .add_custom(&[0x83, 0xEC, 0x1C])
            .add_custom(&[0x57])
            .add_custom(&[0x8B, 0xF8])
            .jump(0x40BE06)
            .inject(0x40BE00);

        // SeedHitTest parameter decreased by 133 (SeedChooserScreen::UpdateCursor)
        Trampoline::new(h_process)
            .add_custom(&[0x81, 0x2C, 0x24, 0x85, 0x00, 0x00, 0x00])
            .call(0x485D80)
            .jump(0x4850B1)
            .inject(0x4850AC);

        // SeedHitTest parameter decreased by 133 (SeedChooserScreen::ShowToolTip)
        Trampoline::new(h_process)
            .add_custom(&[0x81, 0x2C, 0x24, 0x85, 0x00, 0x00, 0x00])
            .call(0x485D80)
            .jump(0x4862FD)
            .inject(0x4862F8);

        // DrawSeedPacket parameter increased by 133 for SEED_IN_BANK state (SeedChooserScreen::Draw)
        Trampoline::new(h_process)
            .add_custom(&[0x05, 0x85, 0x00, 0x00, 0x00])
            .add_custom(&[0x2B, 0x42, 0x30])
            .add_custom(&[0x2B, 0x4A, 0x34])
            .jump(0x484A8B)
            .inject(0x484A85);

        // Set Board->mX to 133 after shake (Board::Update)
        Trampoline::new(h_process)
            .add_custom(&[0xC7, 0x45, 0x30, 0x85, 0x00, 0x00, 0x00])
            .add_custom(&[0x89, 0x5D, 0x34])
            .jump(0x415EE7)
            .inject(0x415EE1);

        // Increase Board->mX by 133 during shake (Board::Update)
        Trampoline::new(h_process)
            .add_custom(&[0x05, 0x85, 0x00, 0x00, 0x00])
            .add_custom(&[0x33, 0xC9])
            .add_custom(&[0x89, 0x45, 0x30])
            .jump(0x415F2F)
            .inject(0x415F2A);

        // min_x -= 133, max_x += 133 during view_lawn (start) (SeedChooserScreen::UpdateViewLawn)
        Trampoline::new(h_process)
            .add_custom(&[0x81, 0x04, 0x24, 0x85, 0x00, 0x00, 0x00])
            .add_custom(&[0x81, 0x6C, 0x24, 0x04, 0x85, 0x00, 0x00, 0x00])
            .call(0x511C40)
            .jump(0x484E4F)
            .inject(0x484E4A);

        // min_x -= 133, max_x += 133 during view_lawn (return) (SeedChooserScreen::UpdateViewLawn)
        Trampoline::new(h_process)
            .add_custom(&[0x81, 0x2C, 0x24, 0x85, 0x00, 0x00, 0x00])
            .add_custom(&[0x81, 0x44, 0x24, 0x04, 0x85, 0x00, 0x00, 0x00])
            .call(0x511C40)
            .jump(0x484F3B)
            .inject(0x484F36);

        // Board->mX = 133, SeedChooserScreen->mX = 133 during view_lawn (view) (SeedChooserScreen::UpdateViewLawn)
        Trampoline::new(h_process)
            .add_custom(&[0xBF, 0x85, 0x00, 0x00, 0x00])
            .add_custom(&[0x57])
            .add_custom(&[0xFF, 0xD2])
            .add_custom(&[0x8B, 0x45, 0x00])
            .jump(0x484ECA)
            .inject(0x484EC4);

        // SeedChooserScreen->mX = 133 during view_lawn (start) (SeedChooserScreen::UpdateViewLawn)
        Trampoline::new(h_process)
            .add_custom(&[0x68, 0x85, 0x00, 0x00, 0x00])
            .add_custom(&[0x8B, 0xCD])
            .add_custom(&[0xFF, 0xD2])
            .add_custom(&[0xC7, 0x04, 0x24, 0x00, 0x00, 0x00, 0x00])
            .jump(0x484E99)
            .inject(0x484E94);

        // SeedChooserScreen->mX = 133 during view_lawn (return) (SeedChooserScreen::UpdateViewLawn)
        Trampoline::new(h_process)
            .add_custom(&[0x68, 0x85, 0x00, 0x00, 0x00])
            .add_custom(&[0x8B, 0xCD])
            .add_custom(&[0xFF, 0xD2])
            .add_custom(&[0xC7, 0x04, 0x24, 0x00, 0x00, 0x00, 0x00])
            .jump(0x484F86)
            .inject(0x484F81);

        Trampoline::new(h_process)
            .add_custom(&[0x81, 0xE9, 0x9E, 0x00, 0x00, 0x00, 0x89, 0x4E, 0x08])
            .jump(0x438805)
            .inject(0x4387FF);

        Trampoline::new(h_process)
            .add_custom(&[0x81, 0xEB, 0x85, 0x00, 0x00, 0x00])
            .add_custom(&[0x53, 0x8B, 0xC5, 0x8B, 0xCE])
            .jump(0x438DEB)
            .inject(0x438DE6);

        Trampoline::new(h_process)
            .add_custom(&[0x8B, 0x90, 0xE0, 0x00, 0x00, 0x00])
            .add_custom(&[0x81, 0xEA, 0x85, 0x00, 0x00, 0x00])
            .jump(0x44833E)
            .inject(0x448338);

        Trampoline::new(h_process)
            .add_custom(&[0x81, 0xC2, 0x85, 0x00, 0x00, 0x00])
            .add_custom(&[0x2B, 0x55, 0x30])
            .add_custom(&[0x2B, 0x45, 0x34])
            .jump(0x448355)
            .inject(0x44834F);

        NtResumeProcess(h_process);
    }
}
