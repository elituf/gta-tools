#![allow(clippy::cast_possible_truncation)]

use crate::util::{self, consts::GTA_WINDOW_TITLE};
use std::time::{Duration, Instant};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    KEYBD_EVENT_FLAGS, MAP_VIRTUAL_KEY_TYPE, MapVirtualKeyW, VK_NUMPAD4, VK_NUMPAD6, keybd_event,
};

pub const INTERVAL: Duration = Duration::from_secs(60);
const PRESS_KEYS: [u8; 2] = [VK_NUMPAD4.0 as u8, VK_NUMPAD6.0 as u8];

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
        use util::win::*;
        is_window_focused(GTA_WINDOW_TITLE)
            && !is_any_key_pressed(&PRESS_KEYS)
            && !is_cursor_visible()
    }

    pub fn activate(&mut self) {
        if self.can_activate() {
            send(&PRESS_KEYS);
        }
        self.interval = Instant::now();
    }
}

pub fn send(vk_codes: &[u8]) {
    vk_codes.iter().for_each(|vk_code: &u8| unsafe {
        keybd_event(
            *vk_code,
            u8::try_from(MapVirtualKeyW(u32::from(*vk_code), MAP_VIRTUAL_KEY_TYPE(0))).unwrap(),
            KEYBD_EVENT_FLAGS(0),
            0,
        );
        keybd_event(
            *vk_code,
            u8::try_from(MapVirtualKeyW(u32::from(*vk_code), MAP_VIRTUAL_KEY_TYPE(0))).unwrap(),
            KEYBD_EVENT_FLAGS(2),
            0,
        );
    });
}
