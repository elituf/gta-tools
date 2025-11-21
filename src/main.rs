#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod features;
mod gui;
mod util;

use std::fmt::Write;

fn init_storage() {
    if !util::consts::path::APP_STORAGE.exists() {
        std::fs::create_dir_all(util::consts::path::APP_STORAGE.as_path()).unwrap();
    }
}

fn panic_hook(panic_info: &std::panic::PanicHookInfo<'_>) {
    let backtrace = std::backtrace::Backtrace::capture();
    let mut message = format!("{panic_info}");
    if backtrace.status() == std::backtrace::BacktraceStatus::Captured {
        write!(message, "\nstack backtrace:\n{backtrace}").unwrap();
    }
    log::error!("{message}");
}

fn main() {
    init_storage();
    util::logging::Logger::init(log::LevelFilter::Info);
    std::panic::set_hook(Box::new(panic_hook));
    gui::run::run();
}
