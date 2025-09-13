use crate::util::{self, consts::game::WINDOW_TITLE};
use std::time::{Duration, Instant};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    INPUT, INPUT_KEYBOARD, KEYBD_EVENT_FLAGS, KEYBDINPUT, KEYEVENTF_KEYUP, MAPVK_VK_TO_VSC,
    MapVirtualKeyW, SendInput, VIRTUAL_KEY, VK_NUMPAD4, VK_NUMPAD6,
};

const INTERVAL: Duration = Duration::from_secs(60);
const PRESS_KEYS: [VIRTUAL_KEY; 2] = [VK_NUMPAD4, VK_NUMPAD6];

#[derive(Debug)]
pub struct AntiAfk {
    pub enabled: bool,
    pub interval: Instant,
}

impl Default for AntiAfk {
    fn default() -> Self {
        Self {
            enabled: false,
            interval: Instant::now(),
        }
    }
}

impl AntiAfk {
    pub fn can_activate(&self) -> bool {
        use util::win::{is_any_key_pressed, is_cursor_visible, is_window_focused};
        is_window_focused(WINDOW_TITLE) && !is_any_key_pressed(&PRESS_KEYS) && !is_cursor_visible()
    }

    pub fn should_activate(&self) -> bool {
        self.enabled && self.interval.elapsed() >= INTERVAL
    }

    pub fn activate(&mut self) {
        send(&PRESS_KEYS);
        self.interval = Instant::now();
    }
}

fn send(vk_codes: &[VIRTUAL_KEY]) {
    let mut inputs = Vec::new();
    for &vk_code in vk_codes {
        let scan_code = unsafe { MapVirtualKeyW(u32::from(vk_code.0), MAPVK_VK_TO_VSC) } as u16;
        for event in [KEYBD_EVENT_FLAGS(0), KEYEVENTF_KEYUP] {
            let mut input = INPUT {
                r#type: INPUT_KEYBOARD,
                ..Default::default()
            };
            input.Anonymous.ki = KEYBDINPUT {
                wVk: vk_code,
                wScan: scan_code,
                dwFlags: event,
                time: 0,
                dwExtraInfo: 0,
            };
            inputs.push(input);
        }
    }
    unsafe { SendInput(&inputs, size_of::<INPUT>() as i32) };
}
