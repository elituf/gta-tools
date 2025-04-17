use windows::{
    Win32::{
        Foundation::{CloseHandle, HANDLE},
        Security::{GetTokenInformation, TOKEN_ELEVATION, TOKEN_QUERY, TokenElevation},
        System::Threading::{GetCurrentProcess, OpenProcessToken},
        UI::{
            Input::KeyboardAndMouse::GetAsyncKeyState,
            Shell::ShellExecuteW,
            WindowsAndMessaging::{
                CURSOR_SHOWING, CURSORINFO, GetCursorInfo, GetForegroundWindow, GetWindowTextW,
                SW_NORMAL,
            },
        },
    },
    core::{HSTRING, PCWSTR},
};

pub enum ElevationExitMethod<'a> {
    Gentle(&'a mut bool),
    Forced,
}

pub fn is_cursor_visible() -> bool {
    let mut ci = CURSORINFO {
        cbSize: u32::try_from(std::mem::size_of::<CURSORINFO>()).unwrap(),
        ..Default::default()
    };
    unsafe {
        GetCursorInfo(&mut ci).unwrap();
    }
    ci.flags == CURSOR_SHOWING
}

#[allow(clippy::cast_sign_loss)]
pub fn is_window_focused(target_title: &str) -> bool {
    let mut buffer: [u16; 512] = [0; 512];
    unsafe {
        let hwnd = GetForegroundWindow();
        let length = GetWindowTextW(hwnd, &mut buffer);
        let current_title = String::from_utf16_lossy(&buffer[..length as usize]);
        current_title == target_title
    }
}

pub fn is_any_key_pressed(keys: &[u8]) -> bool {
    keys.iter()
        .any(|&key| unsafe { (i32::from(GetAsyncKeyState(i32::from(key))) & 0x8000) != 0 })
}

pub fn elevate(closing: ElevationExitMethod) {
    let exe = std::env::current_exe().unwrap();
    unsafe {
        ShellExecuteW(
            None,
            &HSTRING::from("runas"),
            &HSTRING::from(exe.as_path()),
            PCWSTR::null(),
            PCWSTR::null(),
            SW_NORMAL,
        );
    }
    match closing {
        ElevationExitMethod::Gentle(closing) => *closing = true,
        ElevationExitMethod::Forced => std::process::exit(0),
    }
}

pub fn is_elevated() -> bool {
    let mut token: HANDLE = HANDLE::default();
    unsafe {
        if !OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_ok() {
            return false;
        }
        let mut elevation = TOKEN_ELEVATION::default();
        let mut size = u32::try_from(std::mem::size_of::<TOKEN_ELEVATION>()).unwrap();
        let result = GetTokenInformation(
            token,
            TokenElevation,
            Some((&raw mut elevation).cast()),
            size,
            &mut size,
        );
        CloseHandle(token).unwrap();
        result.is_ok() && elevation.TokenIsElevated != 0
    }
}
