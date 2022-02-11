use std::ptr::null_mut;

use winapi::{ctypes::c_void, um::memoryapi::WriteProcessMemory};

use crate::H_PROCESS;

pub unsafe fn patch(address: u32, buf: &[u8]) {
    WriteProcessMemory(
        H_PROCESS,
        address as *mut c_void,
        buf.as_ptr() as *const c_void,
        buf.len(),
        null_mut(),
    );
}
