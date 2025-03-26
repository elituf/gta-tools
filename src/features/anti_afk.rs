use std::time::{Duration, Instant};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    KEYBD_EVENT_FLAGS, MAP_VIRTUAL_KEY_TYPE, MapVirtualKeyW, keybd_event,
};

pub const INTERVAL: Duration = Duration::from_secs(60);
const VK_SHIFT: u8 = 16;

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

pub fn send(vk_code: u8) {
    unsafe {
        keybd_event(
            vk_code,
            u8::try_from(MapVirtualKeyW(u32::from(vk_code), MAP_VIRTUAL_KEY_TYPE(0))).unwrap(),
            KEYBD_EVENT_FLAGS(0),
            0,
        );
        keybd_event(
            vk_code,
            u8::try_from(MapVirtualKeyW(u32::from(vk_code), MAP_VIRTUAL_KEY_TYPE(0))).unwrap(),
            KEYBD_EVENT_FLAGS(2),
            0,
        );
    }
}

pub fn activate() {
    send(VK_SHIFT);
}
