mod trampoline;

use trampoline::Trampoline;

use std::{
    mem::size_of_val,
    os::{
        raw::c_ushort,
        windows::{prelude::AsRawHandle, process::CommandExt},
    },
    process::Command,
    ptr::null_mut,
};

use ntapi::ntpsapi::NtResumeProcess;
use winapi::{ctypes::c_void, um::memoryapi::WriteProcessMemory};

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

        Trampoline::new(h_process)
            .add_custom(&[0xc7, 0x04, 0x24, 0x7b, 0xff, 0xff, 0xff])
            .call(0x587150)
            .jump(0x4164A9)
            .inject(0x4164A4);

        NtResumeProcess(h_process);
    }
}
