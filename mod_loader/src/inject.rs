use std::ffi::CString;
use std::io;
use std::io::Error;
use std::io::ErrorKind;
use std::mem;
use std::path::Path;
use std::ptr;

use log::debug;
use winapi::um::handleapi as whandle;
use winapi::um::libloaderapi as wload;
use winapi::um::memoryapi as wmem;
use winapi::um::processthreadsapi as wproc;
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::winbase::INFINITE;
use winapi::um::winnt::HANDLE;
use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE};

use widestring::WideCString;

macro_rules! werr {
    ($cond:expr) => {
        if $cond {
            let e = io::Error::last_os_error();
            log::error!("windows error: {:?}", e);
            return Err(e);
        }
    };
}

pub(crate) fn inject(proc: HANDLE, dll: &Path) -> io::Result<()> {
    let full_path = dll.canonicalize()?;
    let full_path = full_path.to_string_lossy();
    let full_path = WideCString::from_str(full_path).map_err(|e| {
        Error::new(
            ErrorKind::InvalidInput,
            format!("invalid dll path: {:?}", e),
        )
    })?;

    let path_len = (full_path.len() * 2) + 1;
    // allocate space for the path inside target proc
    let dll_addr = unsafe {
        wmem::VirtualAllocEx(
            proc,
            ptr::null_mut(),
            path_len,
            MEM_RESERVE | MEM_COMMIT,
            PAGE_EXECUTE_READWRITE,
        )
    };

    werr!(dll_addr.is_null());
    debug!("allocated remote memory @ {:?}", dll_addr);

    let res = unsafe {
        // write dll inside target process
        wmem::WriteProcessMemory(
            proc,
            dll_addr,
            full_path.as_ptr() as *mut _,
            path_len,
            ptr::null_mut(),
        )
    };

    werr!(res == 0);

    let krnl = CString::new("Kernel32").unwrap();
    let krnl = unsafe { wload::GetModuleHandleA(krnl.as_ptr()) };
    let loadlib = CString::new("LoadLibraryW").unwrap();
    let loadlib = unsafe { wload::GetProcAddress(krnl, loadlib.as_ptr()) };
    debug!("found LoadLibraryW for injection @ {:?}", loadlib);

    let hthread = unsafe {
        wproc::CreateRemoteThread(
            proc,
            ptr::null_mut(),
            0,
            Some(mem::transmute(loadlib)),
            dll_addr,
            0,
            ptr::null_mut(),
        )
    };

    werr!(hthread.is_null());
    debug!("spawned remote thread @ {:?}", hthread);
    unsafe {
        WaitForSingleObject(hthread, INFINITE);
        whandle::CloseHandle(hthread);
    }

    Ok(())
}
