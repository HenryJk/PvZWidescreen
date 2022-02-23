mod memory;
mod patch_almanac;
mod patch_app;
mod patch_awardscreen;
mod patch_board;
mod patch_bush;
mod patch_button;
mod patch_challenge;
mod patch_cobroofbug;
mod patch_cursorobject;
mod patch_dialogs;
mod patch_fog;
mod patch_gameselector;
mod patch_intro;
mod patch_store;
mod patch_titlescreen;
mod patch_zengarden;

use ntapi::ntpsapi::NtResumeProcess;
use winapi::{ctypes::c_void, um::winnt::PAGE_READWRITE};

use memory::{alloc_mem, patch};
use patch_almanac::patch_almanac;
use patch_app::patch_app;
use patch_awardscreen::patch_awardscreen;
use patch_board::patch_board;
use patch_bush::patch_bush;
use patch_button::patch_button;
use patch_challenge::patch_challenge;
use patch_cobroofbug::patch_cobroofbug;
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
        let global_memory = alloc_mem(24, PAGE_READWRITE) as u32;
        POLE_PTR = global_memory;
        POLE_NIGHT_PTR = global_memory + 8;
        SLOT_MACHINE_OFFSET_PTR = global_memory + 16;
        PAD_CONST_PTR = global_memory + 20;
        patch(PAD_CONST_PTR, &transmute::<i32, [u8; 4]>(PAD as i32));

        patch_almanac()?;
        patch_app()?;
        patch_awardscreen()?;
        patch_board()?;
        patch_bush()?;
        patch_button()?;
        patch_challenge()?;
        patch_cobroofbug()?;
        patch_cursorobject()?;
        patch_dialogs()?;
        patch_gameselector()?;
        patch_fog()?;
        patch_intro()?;
        patch_store()?;
        patch_titlescreen()?;
        patch_zengarden()?;

        // std::thread::sleep(std::time::Duration::from_secs(10));
        NtResumeProcess(H_PROCESS);
    }
    Ok(())
}
