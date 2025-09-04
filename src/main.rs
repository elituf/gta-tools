#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(not(target_os = "windows"))]
compile_error!("This application must be compiled for Windows.");

mod features;
mod gui;
mod util;

fn main() {
    gui::run::run();
}
