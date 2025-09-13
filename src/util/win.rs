use windows::{
    Win32::{
        Foundation::{CloseHandle, HANDLE},
        Security::{GetTokenInformation, TOKEN_ELEVATION, TOKEN_QUERY, TokenElevation},
        System::Threading::{GetCurrentProcess, OpenProcessToken},
        UI::{
            Input::KeyboardAndMouse::{GetAsyncKeyState, VIRTUAL_KEY},
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
    unsafe { GetCursorInfo(&raw mut ci) }.unwrap();
    ci.flags == CURSOR_SHOWING
}

pub fn is_window_focused(target_title: &str) -> bool {
    let mut buffer = [0; 512];
    let hwnd = unsafe { GetForegroundWindow() };
    let length = unsafe { GetWindowTextW(hwnd, &mut buffer) };
    let current_title = String::from_utf16_lossy(&buffer[..length as usize]);
    current_title == target_title
}

pub fn is_any_key_pressed(keys: &[VIRTUAL_KEY]) -> bool {
    keys.iter()
        .any(|&key| unsafe { GetAsyncKeyState(i32::from(key.0)) } & i16::MIN != 0)
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
    if unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &raw mut token) }.is_err() {
        return false;
    }
    let mut elevation = TOKEN_ELEVATION::default();
    let mut size = u32::try_from(std::mem::size_of::<TOKEN_ELEVATION>()).unwrap();
    let result = unsafe {
        GetTokenInformation(
            token,
            TokenElevation,
            Some((&raw mut elevation).cast()),
            size,
            &raw mut size,
        )
    };
    unsafe { CloseHandle(token) }.unwrap();
    result.is_ok() && elevation.TokenIsElevated != 0
}

pub fn is_system_theme_dark() -> bool {
    use winreg::RegKey;
    let hkcu = RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let Ok(subkey) =
        hkcu.open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize")
    else {
        return true;
    };
    let Ok(dword): Result<u32, std::io::Error> = subkey.get_value("AppsUseLightTheme") else {
        return true;
    };
    dword != 1
}
