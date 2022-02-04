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

        Trampoline::new(h_process)
            .add_custom(&[0xC7, 0x44, 0x24, 0x04, 0x7B, 0xFF, 0xFF, 0xFF])
            .call(0x511C40)
            .jump(0x43BA69)
            .inject(0x43BA64);

        Trampoline::new(h_process)
            .add_custom(&[0x6A, 0x00, 0x68, 0x85, 0x00, 0x00, 0x00, 0x89, 0xC1])
            .jump(0x44F661)
            .inject(0x44F65B);

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
