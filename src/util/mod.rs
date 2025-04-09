pub mod consts;
pub mod countdown;
pub mod elevation;

use windows::Win32::UI::{
    Input::KeyboardAndMouse::GetAsyncKeyState,
    WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW},
};

pub fn is_window_focused(target_title: &str) -> bool {
    unsafe {
        let hwnd = GetForegroundWindow();
        let mut buffer: [u16; 512] = [0; 512];
        let length = GetWindowTextW(hwnd, &mut buffer);
        let current_title = String::from_utf16_lossy(&buffer[..length as usize]);
        current_title == target_title
    }
}

pub fn is_key_pressed(key: i32) -> bool {
    unsafe { (GetAsyncKeyState(key) as i32 & 0x8000) != 0 }
}
