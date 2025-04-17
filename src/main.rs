#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod features;
mod gui;
mod util;

fn main() {
    gui::run::run();
}
