use std::{mem::size_of_val, ptr::null_mut};

use winapi::{
    ctypes::c_void,
    um::{
        memoryapi::{VirtualAllocEx, WriteProcessMemory},
        winnt::{MEM_COMMIT, PAGE_EXECUTE_READWRITE},
    },
};

use byteorder::{ByteOrder, LE};

pub struct Trampoline {
    h_process: *mut c_void,
    code: *mut u8,
    next: isize,
}

impl Trampoline {
    pub unsafe fn new(h_process: *mut c_void) -> Self {
        Trampoline {
            h_process,
            code: VirtualAllocEx(
                h_process,
                null_mut(),
                128,
                MEM_COMMIT,
                PAGE_EXECUTE_READWRITE,
            ) as *mut u8,
            next: 0,
        }
    }

    pub unsafe fn add_custom(&mut self, instructions: &[u8]) -> &mut Self {
        WriteProcessMemory(
            self.h_process,
            self.code.offset(self.next) as *mut c_void,
            instructions.as_ptr() as *const c_void,
            instructions.len(),
            null_mut(),
        );
        self.next += instructions.len() as isize;
        self
    }

    pub unsafe fn jump(&mut self, address: u32) -> &mut Self {
        let mut buf = [0; 5];
        buf[0] = 0xE9;
        LE::write_i32(
            &mut buf[1..],
            address as i32 - self.code.offset(self.next + 5) as i32,
        );
        WriteProcessMemory(
            self.h_process,
            self.code.offset(self.next) as *mut c_void,
            (&buf).as_ptr() as *const c_void,
            size_of_val(&buf),
            null_mut(),
        );
        self.next += 5;
        self
    }

    pub unsafe fn call(&mut self, address: u32) -> &mut Self {
        let mut buf = [0; 5];
        buf[0] = 0xE8;
        LE::write_i32(
            &mut buf[1..],
            address as i32 - self.code.offset(self.next + 5) as i32,
        );
        WriteProcessMemory(
            self.h_process,
            self.code.offset(self.next) as *mut c_void,
            (&buf).as_ptr() as *const c_void,
            size_of_val(&buf),
            null_mut(),
        );
        self.next += 5;
        self
    }

    pub unsafe fn inject(&self, address: u32) {
        let mut patch = [0u8; 5];
        patch[0] = 0xE9;
        LE::write_i32(&mut patch[1..], self.code as i32 - (address as i32 + 5));
        WriteProcessMemory(
            self.h_process,
            address as *mut c_void,
            patch.as_ptr() as *const c_void,
            size_of_val(&patch),
            null_mut(),
        );
    }
}
