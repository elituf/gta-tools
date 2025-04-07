use windows::{
    Win32::Foundation::{CloseHandle, HANDLE},
    Win32::Security::{GetTokenInformation, TOKEN_ELEVATION, TOKEN_QUERY, TokenElevation},
    Win32::System::Threading::{GetCurrentProcess, OpenProcessToken},
    Win32::UI::{Shell::ShellExecuteW, WindowsAndMessaging::SW_HIDE},
    core::{HSTRING, PCWSTR},
};

pub fn elevate() {
    let exe = std::env::current_exe().unwrap();
    unsafe {
        ShellExecuteW(
            None,
            &HSTRING::from("runas"),
            &HSTRING::from(exe.as_path()),
            PCWSTR::null(),
            PCWSTR::null(),
            SW_HIDE,
        );
    }
    std::process::exit(0);
}

pub fn is_elevated() -> bool {
    unsafe {
        let mut token: HANDLE = HANDLE::default();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_ok() {
            let mut elevation = TOKEN_ELEVATION::default();
            let size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;
            let mut ret_size = size;
            let result = GetTokenInformation(
                token,
                TokenElevation,
                Some(&mut elevation as *mut _ as *mut _),
                size,
                &mut ret_size,
            );
            CloseHandle(token).unwrap();
            if result.is_ok() && elevation.TokenIsElevated != 0 {
                return true;
            }
        }
        false
    }
}
