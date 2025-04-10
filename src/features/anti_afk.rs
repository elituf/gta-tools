use crate::util::{self, consts::GTA_WINDOW_TITLE};
use std::time::{Duration, Instant};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    KEYBD_EVENT_FLAGS, MAP_VIRTUAL_KEY_TYPE, MapVirtualKeyW, keybd_event,
};

pub const INTERVAL: Duration = Duration::from_secs(60);
const VK_NUMPAD4: u8 = 0x64;
const VK_NUMPAD6: u8 = 0x66;
const PRESS_KEYS: [u8; 2] = [VK_NUMPAD4, VK_NUMPAD6];

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
        if util::win::is_window_focused(GTA_WINDOW_TITLE)
            && !util::win::is_any_key_pressed(&PRESS_KEYS)
        {
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
