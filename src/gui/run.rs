use crate::{
    gui::{
        app::{App, WINDOW_SIZE},
        tools,
    },
    util::{self, consts::APP_STORAGE_PATH, persistent_state::PersistentState},
};
use eframe::egui;
use std::{fs::File, io::Write};

fn panic_hook(panic_info: &std::panic::PanicHookInfo<'_>) {
    let log_path = APP_STORAGE_PATH.join("panic.log");
    let mut file = File::options()
        .create(true)
        .append(true)
        .open(log_path)
        .unwrap();
    let timestamp = chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false);
    let backtrace = std::backtrace::Backtrace::force_capture();
    let message = format!("[{timestamp}]\n{panic_info}\nstack backtrace:\n{backtrace}\n");
    file.write_all(message.as_bytes()).unwrap();
}

#[allow(clippy::unnecessary_wraps)]
fn app_creator(
    cc: &eframe::CreationContext<'_>,
) -> Result<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>> {
    std::panic::set_hook(Box::new(panic_hook));
    let mut app = Box::<App>::default();
    if let Some(persistent_state) = PersistentState::get() {
        app.launch.selected = persistent_state.launcher;
        app.settings = persistent_state.settings;
    }
    let elevated = util::win::is_elevated();
    if app.settings.start_elevated && !elevated {
        util::win::elevate(util::win::ElevationExitMethod::Forced);
    }
    app.flags.elevated = elevated;
    egui_extras::install_image_loaders(&cc.egui_ctx);
    cc.egui_ctx.style_mut(|style| {
        style.spacing.item_spacing = egui::vec2(4.0, 4.0);
        style.interaction.selectable_labels = false;
    });
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "Ubuntu-Regular".to_owned(),
        egui::FontData::from_static(include_bytes!("../../assets/Ubuntu-Regular.ttf")).into(),
    );
    fonts.families.insert(
        egui::FontFamily::Name("Ubuntu-Regular".into()),
        vec!["Ubuntu-Regular".to_owned()],
    );
    cc.egui_ctx.set_fonts(fonts);
    Ok(app)
}

pub fn run() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(false)
            .with_maximize_button(false)
            .with_inner_size(WINDOW_SIZE)
            .with_icon(tools::load_icon()),
        centered: true,
        ..Default::default()
    };
    eframe::run_native("GTA Tools", options, Box::new(app_creator)).unwrap();
}
