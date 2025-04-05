use windows::{
    Win32::UI::{Shell::ShellExecuteW, WindowsAndMessaging::SW_HIDE},
    core::{HSTRING, PCWSTR},
};

pub fn elevate() {
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
    std::process::exit(0);
}
