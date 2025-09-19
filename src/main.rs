// main.rs
#![windows_subsystem = "windows"]

use std::process;
use winapi::um::winnt::{OSVERSIONINFOEXW};
use winapi::um::sysinfoapi::GetVersionExW;
use winapi::shared::minwindef::DWORD;
use std::mem;
use std::ptr;
use web_view::*;

fn get_windows_version() -> (DWORD, DWORD) {
    unsafe {
        let mut os_info: OSVERSIONINFOEXW = mem::zeroed();
        os_info.dwOSVersionInfoSize = mem::size_of::<OSVERSIONINFOEXW>() as DWORD;
        
        if GetVersionExW(&mut os_info as *mut OSVERSIONINFOEXW as *mut _) != 0 {
            (os_info.dwMajorVersion, os_info.dwMinorVersion)
        } else {
            // Fallback for newer Windows versions that may not report correctly
            // Try alternative detection method
            detect_windows_version_alternative()
        }
    }
}

fn detect_windows_version_alternative() -> (DWORD, DWORD) {
    // Windows versions: 
    // Windows 7: 6.1
    // Windows 8: 6.2
    // Windows 8.1: 6.3
    // Windows 10: 10.0
    // Windows 11: 10.0 (build 22000+)
    
    // Check using RtlGetVersion for more accurate results
    use winapi::um::winnt::RTL_OSVERSIONINFOEXW;
    use winapi::shared::ntdef::NTSTATUS;
    
    type RtlGetVersionFn = unsafe extern "system" fn(*mut RTL_OSVERSIONINFOEXW) -> NTSTATUS;
    
    unsafe {
        let ntdll = winapi::um::libloaderapi::GetModuleHandleW(
            "ntdll.dll\0".encode_utf16().collect::<Vec<u16>>().as_ptr()
        );
        
        if !ntdll.is_null() {
            let rtl_get_version = winapi::um::libloaderapi::GetProcAddress(
                ntdll,
                "RtlGetVersion\0".as_ptr() as *const i8
            );
            
            if !rtl_get_version.is_null() {
                let rtl_get_version_fn: RtlGetVersionFn = mem::transmute(rtl_get_version);
                let mut os_info: RTL_OSVERSIONINFOEXW = mem::zeroed();
                os_info.dwOSVersionInfoSize = mem::size_of::<RTL_OSVERSIONINFOEXW>() as DWORD;
                
                if rtl_get_version_fn(&mut os_info) == 0 {
                    return (os_info.dwMajorVersion, os_info.dwMinorVersion);
                }
            }
        }
    }
    
    (6, 1) // Default to Windows 7 if detection fails
}

fn show_compatibility_error(version_string: &str) {
    use winapi::um::winuser::{MessageBoxW, MB_OK, MB_ICONERROR};
    
    let title = "Forlenza Industrial SCADA v2.1 - Compatibility Error"
        .encode_utf16()
        .chain(Some(0))
        .collect::<Vec<u16>>();
    
    let message = format!(
        "Forlenza Industrial SCADA v2.1 requires Windows 7 or earlier.\n\n\
        This software is not compatible with {}.\n\n\
        Please contact IT support for virtualization solutions.\n\n\
        Error Code: LEGACY_OS_REQUIRED",
        version_string
    )
    .encode_utf16()
    .chain(Some(0))
    .collect::<Vec<u16>>();
    
    unsafe {
        MessageBoxW(
            ptr::null_mut(),
            message.as_ptr(),
            title.as_ptr(),
            MB_OK | MB_ICONERROR
        );
    }
}

fn get_version_string(major: DWORD, minor: DWORD) -> String {
    match (major, minor) {
        (6, 2) => "Windows 8".to_string(),
        (6, 3) => "Windows 8.1".to_string(),
        (10, 0) => "Windows 10/11".to_string(),
        (v1, v2) if v1 > 6 || (v1 == 6 && v2 > 1) => format!("Windows {}.{}", v1, v2),
        _ => "Windows 7".to_string(),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dev_mode = args.iter().any(|arg| arg == "--dev");

    let (major, minor) = get_windows_version();

    // Check if running on Windows 8 or higher, unless in dev mode
    if !dev_mode && (major > 6 || (major == 6 && minor > 1)) {
        let version_string = get_version_string(major, minor);
        show_compatibility_error(&version_string);
        process::exit(1);
    }

    // If we're here, we're on Windows 7 or earlier, or dev mode is enabled - run the SCADA application
    let html_content = include_str!("scada_ui.html");

    web_view::builder()
        .title("Forlenza Industrial SCADA Control System v2.1")
        .content(Content::Html(html_content))
        .size(1280, 800)
        .resizable(true)
        .debug(false)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}