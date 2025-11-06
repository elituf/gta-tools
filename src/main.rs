#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod features;
mod gui;
mod util;

fn init_storage() {
    if !util::consts::path::APP_STORAGE.exists() {
        std::fs::create_dir_all(util::consts::path::APP_STORAGE.as_path()).unwrap();
    }
}

fn panic_hook(panic_info: &std::panic::PanicHookInfo<'_>) {
    let backtrace = std::backtrace::Backtrace::capture();
    let message = format!("{panic_info}\nstack backtrace:\n{backtrace}\n");
    util::log::log(util::log::LogLevel::Panic, &message);
}

fn main() {
    init_storage();
    std::panic::set_hook(Box::new(panic_hook));
    gui::run::run();
}
