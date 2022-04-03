#![allow(dead_code)]

use core::{
    ffi::c_void,
    mem::zeroed,
    ptr::{copy, null_mut},
};

use iced_x86::code_asm::CodeAssembler;
use windows::Win32::System::Memory::{
    VirtualAlloc, VirtualProtect, MEM_COMMIT, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS,
};

pub(crate) unsafe fn alloc_mem(size: usize, permission: PAGE_PROTECTION_FLAGS) -> *mut c_void {
    VirtualAlloc(null_mut(), size, MEM_COMMIT, permission)
}

pub(crate) unsafe fn change_permission(
    address: u32,
    size: usize,
    permission: PAGE_PROTECTION_FLAGS,
) {
    let mut old_perm: PAGE_PROTECTION_FLAGS = zeroed();
    VirtualProtect(address as *const c_void, size, permission, &mut old_perm);
}

pub(crate) unsafe fn patch(address: u32, buf: &[u8]) {
    change_permission(address, buf.len(), PAGE_EXECUTE_READWRITE);
    copy(buf.as_ptr(), address as *mut u8, buf.len());
}

pub(crate) unsafe fn inject(address: u32, mut code: CodeAssembler) {
    let code_length = code.assemble(0).unwrap().len();
    let exec_mem_address = alloc_mem(code_length + 5, PAGE_EXECUTE_READWRITE);

    let buf = code.assemble(exec_mem_address as u64).unwrap();
    patch(exec_mem_address as u32, &buf);

    let mut code = CodeAssembler::new(32).unwrap();
    code.jmp(exec_mem_address as u64).unwrap();

    let buf = code.assemble(address as u64).unwrap();
    patch(address, &buf);
}
