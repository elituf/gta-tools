use crate::{
    gui::{
        app::{App, WINDOW_SIZE},
        tools,
    },
    util::{consts::APP_STORAGE_PATH, persistent_state::PersistentState, win},
};
use eframe::egui;
use std::{
    fs::File,
    io::Write,
    time::{SystemTime, UNIX_EPOCH},
};

fn panic_hook(panic_info: &std::panic::PanicHookInfo<'_>) {
    let log_path = APP_STORAGE_PATH.join("panic.log");
    let mut file = File::options()
        .create(true)
        .append(true)
        .open(log_path)
        .unwrap();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let backtrace = std::backtrace::Backtrace::capture();
    let message = format!("[{timestamp}]\n{panic_info}\nstack backtrace:\n{backtrace}\n");
    file.write_all(message.as_bytes()).unwrap();
}

fn app_creator(
    cc: &eframe::CreationContext<'_>,
) -> Result<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>> {
    // use our own panic hook which logs all panics to a file
    std::panic::set_hook(Box::new(panic_hook));
    // initialize http client (nyquest) for windows
    nyquest_backend_winrt::register();
    // initialize App early to modify some things before returning it
    let mut app = Box::new(App::default());
    // load previously selected launch platform & settings from persistent state
    if let Some(persistent_state) = PersistentState::get() {
        app.launch.selected = persistent_state.launcher;
        app.settings = persistent_state.settings;
    }
    // check if we're elevated. if not, and the user wants an elevated launch - relaunch elevated
    if !app.flags.elevated && app.settings.start_elevated {
        win::elevate(win::ElevationExitMethod::Forced);
    }
    // refresh sysinfo because it initializes with nothing
    app.sysinfo.refresh_all();
    // enable image loading support in egui
    egui_extras::install_image_loaders(&cc.egui_ctx);
    // set our initial theme, from earlier loaded settings. we set the egui theme
    // to dark here to work around system theme based switching of the egui style
    cc.egui_ctx.set_theme(egui::Theme::Dark);
    catppuccin_egui::set_theme(&cc.egui_ctx, app.settings.theme.into());
    // apply some global styling that we like
    cc.egui_ctx.all_styles_mut(|style| {
        style.spacing.item_spacing = egui::vec2(4.0, 4.0);
        style.interaction.selectable_labels = false;
    });
    // load any extra fonts that we need
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
    // finally return the App
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
