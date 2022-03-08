mod memory;
mod patch_almanac;
mod patch_app;
mod patch_awardscreen;
mod patch_board;
mod patch_bush;
mod patch_button;
mod patch_challenge;
mod patch_cobroofbug;
mod patch_credits;
mod patch_cursorobject;
mod patch_dialogs;
mod patch_fog;
mod patch_gameselector;
mod patch_intro;
mod patch_store;
mod patch_titlescreen;
mod patch_zengarden;

use winapi::um::winnt::{DLL_PROCESS_ATTACH, PAGE_READWRITE};

use memory::{alloc_mem, patch};
use patch_almanac::patch_almanac;
use patch_app::patch_app;
use patch_awardscreen::patch_awardscreen;
use patch_board::patch_board;
use patch_bush::patch_bush;
use patch_button::patch_button;
use patch_challenge::patch_challenge;
use patch_cobroofbug::patch_cobroofbug;
use patch_credits::patch_credits;
use patch_cursorobject::patch_cursorobject;
use patch_dialogs::patch_dialogs;
use patch_fog::patch_fog;
use patch_gameselector::patch_gameselector;
use patch_intro::patch_intro;
use patch_store::patch_store;
use patch_titlescreen::patch_titlescreen;
use patch_zengarden::patch_zengarden;

use core::intrinsics::transmute;

const PAD: i16 = 133;
const POLE_OFFSET: i16 = 27;

static mut POLE_PTR: u32 = 0;
static mut POLE_NIGHT_PTR: u32 = 0;
static mut SLOT_MACHINE_OFFSET_PTR: u32 = 0;
static mut PAD_CONST_PTR: u32 = 0;

#[no_mangle] // call it "DllMain" in the compiled DLL
#[allow(unused_variables)]
pub extern "stdcall" fn DllMain(
    hinst_dll: winapi::shared::minwindef::HINSTANCE,
    fdw_reason: winapi::shared::minwindef::DWORD,
    lpv_reserved: winapi::shared::minwindef::LPVOID,
) -> i32 {
    match fdw_reason {
        DLL_PROCESS_ATTACH => {
            unsafe {
                // H_PROCESS = pvz_process.as_raw_handle() as *mut c_void;
                let global_memory = alloc_mem(24, PAGE_READWRITE) as u32;
                POLE_PTR = global_memory;
                POLE_NIGHT_PTR = global_memory + 8;
                SLOT_MACHINE_OFFSET_PTR = global_memory + 16;
                PAD_CONST_PTR = global_memory + 20;
                patch(PAD_CONST_PTR, &transmute::<i32, [u8; 4]>(PAD as i32));

                patch_almanac().unwrap();
                patch_app().unwrap();
                patch_awardscreen().unwrap();
                patch_board().unwrap();
                patch_bush().unwrap();
                patch_button().unwrap();
                patch_challenge().unwrap();
                patch_cobroofbug().unwrap();
                patch_credits().unwrap();
                patch_cursorobject().unwrap();
                patch_dialogs().unwrap();
                patch_gameselector().unwrap();
                patch_fog().unwrap();
                patch_intro().unwrap();
                patch_store().unwrap();
                patch_titlescreen().unwrap();
                patch_zengarden().unwrap();
            }
            return true as i32;
        }
        _ => true as i32,
    }
}
