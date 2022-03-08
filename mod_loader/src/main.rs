mod config;
mod inject;

use std::{
    ffi::OsStr,
    io::{stderr, stdin, stdout},
    iter, mem,
    os::windows::prelude::{AsRawHandle, OsStrExt},
    path::Path,
    ptr::{null, null_mut},
};

use config::Config;
use pelite::pe32::{Pe, PeView};
use pelite::ImageMap;

use inject::inject;
use winapi::{
    ctypes::c_void,
    shared::minwindef::DWORD,
    um::{
        memoryapi::{ReadProcessMemory, WriteProcessMemory},
        processthreadsapi::{
            CreateProcessW, GetThreadContext, ResumeThread, SuspendThread, PROCESS_INFORMATION,
            STARTUPINFOW,
        },
        winbase::{CREATE_NO_WINDOW, CREATE_SUSPENDED},
        winnt::{CONTEXT, CONTEXT_CONTROL},
    },
};

// OsStr to zero-terminated owned vector
fn to_nullterm(s: &OsStr) -> Vec<u16> {
    s.encode_wide().chain(iter::once(0u16)).collect()
}

fn main() {
    let config = Config::get_config();

    let mut sinfo: STARTUPINFOW = unsafe { mem::zeroed() };
    sinfo.cb = mem::size_of::<STARTUPINFOW>() as DWORD;
    sinfo.hStdInput = stdin().as_raw_handle() as *mut c_void;
    sinfo.hStdOutput = stdout().as_raw_handle() as *mut c_void;
    sinfo.hStdError = stderr().as_raw_handle() as *mut c_void;
    let mut pinfo: PROCESS_INFORMATION = unsafe { mem::zeroed() };
    let appname = &config
        .executable
        .unwrap_or("PlantsVsZombies.exe".to_string());
    let wc_appname = to_nullterm(OsStr::new(&appname));
    let creation_flags = CREATE_SUSPENDED | CREATE_NO_WINDOW;
    unsafe {
        CreateProcessW(
            wc_appname.as_ptr(),
            null_mut(),
            null_mut(),
            null_mut(),
            false as i32,
            creation_flags,
            null_mut(),
            null(),
            &mut sinfo,
            &mut pinfo,
        );
    }
    let image = ImageMap::open(&appname).unwrap();
    let header = PeView::from_bytes(&image).unwrap().optional_header();

    let entry_point = (header.ImageBase + header.AddressOfEntryPoint) as *mut c_void;

    let mut original_code = [0u8; 2];

    unsafe {
        ReadProcessMemory(
            pinfo.hProcess,
            entry_point,
            original_code.as_mut_ptr() as *mut c_void,
            2,
            null_mut(),
        );

        WriteProcessMemory(
            pinfo.hProcess,
            entry_point,
            [0xEBu8, 0xFE].as_ptr() as *const c_void,
            2,
            null_mut(),
        );

        ResumeThread(pinfo.hThread);

        let mut context: CONTEXT = mem::zeroed();

        for _ in 0..500 {
            std::thread::sleep(std::time::Duration::from_millis(10));

            // read the thread context
            context.ContextFlags = CONTEXT_CONTROL;
            GetThreadContext(pinfo.hThread, &mut context);

            if context.Eip == entry_point as u32 {
                break;
            }
        }

        for module in config.mods.unwrap_or(vec![]) {
            let dllpath = Path::new(&module);
            match inject(pinfo.hProcess, dllpath) {
                Ok(()) => {}
                Err(_) => {
                    println!("{:?} failed to inject", dllpath);
                }
            }
        }

        SuspendThread(pinfo.hThread);

        WriteProcessMemory(
            pinfo.hProcess,
            entry_point,
            original_code.as_ptr() as *const c_void,
            2,
            null_mut(),
        );

        // std::thread::sleep(std::time::Duration::from_secs(20));
        ResumeThread(pinfo.hThread);

        // WaitForSingleObject(pinfo.hProcess, INFINITE);
    }
}
