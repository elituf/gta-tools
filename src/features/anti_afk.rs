use crate::util::{self, consts::game::WINDOW_TITLE};
use std::time::{Duration, Instant};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    KEYBD_EVENT_FLAGS, MAP_VIRTUAL_KEY_TYPE, MapVirtualKeyW, VIRTUAL_KEY, VK_NUMPAD4, VK_NUMPAD6,
    keybd_event,
};

pub const INTERVAL: Duration = Duration::from_secs(60);
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
    pub fn activate(&mut self) {
        if can_activate() {
            send(&PRESS_KEYS);
        }
        self.interval = Instant::now();
    }
}

pub fn can_activate() -> bool {
    use util::win::{is_any_key_pressed, is_cursor_visible, is_window_focused};
    is_window_focused(WINDOW_TITLE) && !is_any_key_pressed(&PRESS_KEYS) && !is_cursor_visible()
}

pub fn send(vk_codes: &[VIRTUAL_KEY]) {
    vk_codes.iter().for_each(|vk_code| unsafe {
        keybd_event(
            vk_code.0 as u8,
            u8::try_from(MapVirtualKeyW(
                u32::from(vk_code.0),
                MAP_VIRTUAL_KEY_TYPE(0),
            ))
            .unwrap(),
            KEYBD_EVENT_FLAGS(0),
            0,
        );
        keybd_event(
            vk_code.0 as u8,
            u8::try_from(MapVirtualKeyW(
                u32::from(vk_code.0),
                MAP_VIRTUAL_KEY_TYPE(0),
            ))
            .unwrap(),
            KEYBD_EVENT_FLAGS(2),
            0,
        );
    });
}
