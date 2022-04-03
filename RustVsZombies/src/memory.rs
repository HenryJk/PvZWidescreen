#![allow(dead_code)]

use core::ptr::null_mut;

use byteorder::{ByteOrder, LE};
use iced_x86::code_asm::CodeAssembler;
use winapi::{
    ctypes::c_void,
    um::{
        memoryapi::{VirtualAlloc, VirtualProtect, WriteProcessMemory},
        processthreadsapi::GetCurrentProcess,
        winnt::{MEM_COMMIT, PAGE_EXECUTE_READWRITE},
    },
};

pub(crate) unsafe fn alloc_mem(size: usize, permission: u32) -> *mut c_void {
    VirtualAlloc(null_mut(), size, MEM_COMMIT, permission)
}

pub(crate) unsafe fn change_permission(address: u32, size: usize, permission: u32) {
    let mut old_perm = 0u32;
    VirtualProtect(
        address as *mut c_void,
        size,
        permission,
        &mut old_perm as *mut u32,
    );
}

pub(crate) unsafe fn patch(address: u32, buf: &[u8]) {
    WriteProcessMemory(
        GetCurrentProcess(),
        address as *mut c_void,
        buf.as_ptr() as *const c_void,
        buf.len(),
        null_mut(),
    );
}

pub(crate) unsafe fn inject(address: u32, mut code: CodeAssembler) {
    let code_length = code.assemble(0).unwrap().len();
    let exec_mem_address = alloc_mem(code_length + 5, PAGE_EXECUTE_READWRITE);

    let buf = code.assemble(exec_mem_address as u64).unwrap();
    WriteProcessMemory(
        GetCurrentProcess(),
        exec_mem_address,
        buf.as_ptr() as *const c_void,
        buf.len(),
        null_mut(),
    );

    let mut patch = [0u8; 5];
    patch[0] = 0xE9;
    LE::write_i32(
        &mut patch[1..],
        exec_mem_address as i32 - (address as i32 + 5),
    );
    WriteProcessMemory(
        GetCurrentProcess(),
        address as *mut c_void,
        patch.as_ptr() as *const c_void,
        patch.len(),
        null_mut(),
    );
}
