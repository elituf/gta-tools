use windows::{
    Win32::{
        Foundation::{CloseHandle, HANDLE},
        Security::{GetTokenInformation, TOKEN_ELEVATION, TOKEN_QUERY, TokenElevation},
        System::Threading::{GetCurrentProcess, OpenProcessToken},
        UI::{
            Input::KeyboardAndMouse::GetAsyncKeyState,
            Shell::ShellExecuteW,
            WindowsAndMessaging::SW_HIDE,
            WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW},
        },
    },
    core::{HSTRING, PCWSTR},
};

#[allow(clippy::cast_sign_loss)]
pub fn is_window_focused(target_title: &str) -> bool {
    unsafe {
        let hwnd = GetForegroundWindow();
        let mut buffer: [u16; 512] = [0; 512];
        let length = GetWindowTextW(hwnd, &mut buffer);
        let current_title = String::from_utf16_lossy(&buffer[..length as usize]);
        current_title == target_title
    }
}

pub fn is_any_key_pressed(keys: &[u8]) -> bool {
    keys.iter()
        .any(|&key| unsafe { (i32::from(GetAsyncKeyState(i32::from(key))) & 0x8000) != 0 })
}

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
