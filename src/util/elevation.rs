use windows::{
    Win32::Foundation::{CloseHandle, HANDLE},
    Win32::Security::{GetTokenInformation, TOKEN_ELEVATION, TOKEN_QUERY, TokenElevation},
    Win32::System::Threading::{GetCurrentProcess, OpenProcessToken},
    Win32::UI::{Shell::ShellExecuteW, WindowsAndMessaging::SW_HIDE},
    core::{HSTRING, PCWSTR},
};

pub fn elevate(closing: &mut bool) {
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
    *closing = true;
}

pub fn is_elevated() -> bool {
    unsafe {
        let mut token: HANDLE = HANDLE::default();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_ok() {
            let mut elevation = TOKEN_ELEVATION::default();
            let size = u32::try_from(std::mem::size_of::<TOKEN_ELEVATION>()).unwrap();
            let mut ret_size = size;
            let result = GetTokenInformation(
                token,
                TokenElevation,
                Some((&raw mut elevation).cast()),
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
