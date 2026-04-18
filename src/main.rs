use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot,
    Process32FirstW,
    Process32NextW,
    Module32FirstW,
    Module32NextW,
    PROCESSENTRY32W,
    MODULEENTRY32W,
    TH32CS_SNAPPROCESS,
    TH32CS_SNAPMODULE,
    TH32CS_SNAPMODULE32,
};
use windows_sys::Win32::System::Threading::{
    OpenProcess,
    PROCESS_QUERY_INFORMATION,
    PROCESS_VM_READ,
};

fn wstr(buf: &[u16]) -> String {
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    OsString::from_wide(&buf[..len]).to_string_lossy().into_owned()
}

fn get_pid(name: &str) -> Option<u32> {
    unsafe {
        let snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snap == INVALID_HANDLE_VALUE {
            return None;
        }

        let mut entry: PROCESSENTRY32W = std::mem::zeroed();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

        let mut result = None;

        if Process32FirstW(snap, &mut entry) != 0 {
            loop {
                let pname = wstr(&entry.szExeFile);

                if pname.eq_ignore_ascii_case(name) {
                    result = Some(entry.th32ProcessID);
                    break;
                }

                if Process32NextW(snap, &mut entry) == 0 {
                    break;
                }
            }
        }

        CloseHandle(snap);
        result
    }
}

fn list_modules(pid: u32) {
    unsafe {
        let snap = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, pid);
        if snap == INVALID_HANDLE_VALUE {
            println!("module snapshot failed");
            return;
        }

        let mut entry: MODULEENTRY32W = std::mem::zeroed();
        entry.dwSize = std::mem::size_of::<MODULEENTRY32W>() as u32;

        if Module32FirstW(snap, &mut entry) == 0 {
            CloseHandle(snap);
            return;
        }

        println!("{:<40} | BASE ADDRESS", "MODULE");
        println!("{:-<80}", "-");

        loop {
            let name = wstr(&entry.szModule);
            println!("{:<40} | {:p}", name, entry.modBaseAddr);

            if Module32NextW(snap, &mut entry) == 0 {
                break;
            }
        }

        CloseHandle(snap);
    }
}

fn main() {
    let target = "test.exe";

    println!("scanning: {}", target);

    let pid = match get_pid(target) {
        Some(p) => p,
        None => {
            println!("process not found");
            return;
        }
    };

    println!("pid: {}", pid);

    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid);

        if handle == 0 {
            println!("access denied");
            return;
        }

        println!("access granted");

        list_modules(pid);

        CloseHandle(handle);
    }
}
