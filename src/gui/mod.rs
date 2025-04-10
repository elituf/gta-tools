mod persistent_state;
mod settings;

use crate::{
    features::{
        self, anti_afk::AntiAfk, empty_session::EmptySession, force_close::ForceClose,
        launch::Launch,
    },
    gui::{persistent_state::PersistentState, settings::Settings},
    util::{
        self,
        consts::{ENHANCED, GTA_WINDOW_TITLE, LEGACY},
    },
};
use eframe::egui;
use std::time::{Duration, Instant};

const WINDOW_SIZE: [f32; 2] = [240.0, 245.0];

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Stage {
    #[default]
    Main,
    Settings,
    About,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Default)]
pub struct Flags {
    elevated: bool,
    debug: bool,
    closing: bool,
    current_frame: bool,
}

#[derive(Debug)]
pub struct App {
    settings: Settings,
    stage: Stage,
    flags: Flags,
    pub sysinfo: sysinfo::System,
    pub game_handle: windows::Win32::Foundation::HANDLE,
    launch: Launch,
    force_close: ForceClose,
    empty_session: EmptySession,
    anti_afk: AntiAfk,
}

impl Default for App {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            stage: Stage::default(),
            flags: Flags::default(),
            sysinfo: sysinfo::System::new_all(),
            game_handle: windows::Win32::Foundation::HANDLE::default(),
            launch: Launch::default(),
            force_close: ForceClose::default(),
            empty_session: EmptySession::default(),
            anti_afk: AntiAfk::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        catppuccin_egui::set_theme(ctx, self.settings.theme.into());
        self.run_timers();
        egui::TopBottomPanel::bottom("bottom_panel")
            .exact_height(25.0)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.selectable_value(&mut self.stage, Stage::Main, "Main");
                    ui.selectable_value(&mut self.stage, Stage::Settings, "Settings");
                    ui.selectable_value(&mut self.stage, Stage::About, "About");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let button = ui
                            .add_enabled(!self.flags.elevated, egui::Button::new("Elevate"))
                            .on_hover_text("Relaunch ourselves as administrator.")
                            .on_disabled_hover_text("We are already running elevated.");
                        if button.clicked() {
                            util::win::elevate(util::win::ElevationExitMethod::Gentle(
                                &mut self.flags.closing,
                            ));
                        }
                    });
                });
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, true])
                .show(ui, |ui| match self.stage {
                    Stage::Main => {
                        self.header(ui, "Game");
                        self.show_game(ctx, ui);
                        self.header(ui, "Session");
                        self.show_session(ctx, ui);
                        self.header(ui, "Network");
                        self.show_network(ctx, ui);
                    }
                    Stage::Settings => self.show_settings(ctx, ui),
                    Stage::About => self.show_about(ctx, ui),
                });
        });
        if check_debug_keycombo_pressed(ctx) {
            self.flags.debug = !self.flags.debug;
        }
        if check_debug_viewport_close_button_pressed(ctx) {
            self.flags.debug = false;
        }
        if self.flags.debug {
            let main_rect = ctx.input(|i| {
                i.viewport()
                    .clone()
                    .outer_rect
                    .unwrap_or(egui::Rect::EVERYTHING)
            });
            let position = [main_rect.right(), main_rect.min.y];
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("debug_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("GTA Tools Debug")
                    .with_minimize_button(false)
                    .with_maximize_button(false)
                    .with_inner_size(WINDOW_SIZE)
                    .with_position(position)
                    .with_icon(load_icon()),
                |ctx, _class| {
                    if check_debug_keycombo_pressed(ctx) {
                        self.flags.debug = !self.flags.debug;
                    }
                    egui::CentralPanel::default().show(ctx, |ui| {
                        egui::ScrollArea::both()
                            .auto_shrink([false, true])
                            .show(ui, |ui| {
                                self.show_debug(ctx, ui);
                            });
                    });
                },
            );
        }
        ctx.request_repaint_after(Duration::from_millis(100));
        if self.flags.closing {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}

impl App {
    #[allow(clippy::unused_self)]
    fn header(&self, ui: &mut egui::Ui, text: &str) {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(text).font(egui::FontId::new(
                12.5,
                egui::FontFamily::Name("Ubuntu-Regular".into()),
            )));
            ui.add(egui::Separator::default().horizontal());
        });
    }

    fn show_game(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Launch").clicked() {
                features::launch::launch(&self.launch.selected);
            };
            build_combo_box::<features::launch::Platform>(ui, &mut self.launch.selected, "Launch");
        });
        let force_close_button = ui.add_sized(
            [104.0, 0.0],
            egui::Button::new(&self.force_close.button_text),
        );
        if force_close_button.clicked() && !self.force_close.prompting {
            self.force_close.prompting();
            self.flags.current_frame = true;
        };
        if self.force_close.prompting
            && self.force_close.interval.elapsed() <= Duration::from_secs(3)
        {
            if force_close_button.clicked() && !self.flags.current_frame {
                features::force_close::activate(&mut self.sysinfo);
                self.force_close = ForceClose::default();
            }
        } else {
            self.force_close = ForceClose::default();
        }
        if self.flags.current_frame {
            self.flags.current_frame = false;
        }
    }

    fn show_session(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.add_enabled_ui(!self.empty_session.disabled, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Empty current session").clicked() {
                    self.empty_session.interval = Instant::now();
                    self.empty_session.disabled = true;
                    features::empty_session::activate(self);
                }
                ui.label(&self.empty_session.countdown.i_string);
            });
        });
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.anti_afk.enabled, "Anti AFK")
                .on_hover_text("You should be tabbed in\nfor this to work.");
            if self.anti_afk.enabled {
                ui.add_space(8.0);
                ui.add_enabled_ui(false, |ui| {
                    ui.label(if util::win::is_window_focused(GTA_WINDOW_TITLE) {
                        "GTA is focused."
                    } else {
                        "GTA is not focused!"
                    })
                });
            }
        });
        if self.anti_afk.enabled && self.anti_afk.interval.elapsed() >= features::anti_afk::INTERVAL
        {
            self.anti_afk.activate();
        }
    }

    fn show_network(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::Frame::new()
            .inner_margin(egui::vec2(4.0, 4.0))
            .stroke(egui::Stroke::new(
                1.0,
                self.settings.theme.to_catppuccin().overlay1,
            ))
            .show(ui, |ui| {
                let response = ui.add_enabled_ui(self.flags.elevated, |ui| {
                    let label = ui.label("Game's network access");
                    ui.horizontal(|ui| {
                        let available_width = label.rect.width();
                        let spacing = ui.spacing().item_spacing.x;
                        let button_width = (available_width - spacing) / 2.0;
                        if ui
                            .add_sized([button_width, 18.0], egui::Button::new("Block"))
                            .clicked()
                        {
                            features::game_networking::block_all(&mut self.sysinfo);
                        };
                        if ui
                            .add_sized([button_width, 18.0], egui::Button::new("Unblock"))
                            .clicked()
                        {
                            features::game_networking::unblock_all();
                        };
                    });
                });
                response.response.on_disabled_hover_text(
                    "This requires administrator.\nUse the Elevate button.",
                );
            });
    }

    #[allow(clippy::unused_self)]
    fn show_about(&self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            ui.horizontal(|ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("with love from ");
                    ui.hyperlink_to("futile", "http://futile.eu");
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!(
                        "v{} {}",
                        env!("CARGO_PKG_VERSION"),
                        if cfg!(debug_assertions) { "(dev)" } else { "" }
                    ));
                });
            });
            ui.add(egui::Image::new(egui::include_image!(
                "../../assets/icon.png"
            )));
        });
    }

    fn show_settings(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Theme");
            build_combo_box::<settings::Theme>(ui, &mut self.settings.theme, "Theme");
        });
        ui.checkbox(&mut self.settings.start_elevated, "Always start elevated");
    }

    fn show_debug(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.collapsing("times", |ui| {
            ui.label(format!(
                "anti afk timer: {}",
                self.anti_afk.interval.elapsed().as_secs()
            ))
        });
        ui.collapsing("sysinfo", |ui| {
            if ui.button("refresh all").clicked() {
                self.sysinfo.refresh_all();
            }
            let pid = self
                .sysinfo
                .processes()
                .iter()
                .find(|(_, p)| p.name() == ENHANCED || p.name() == LEGACY)
                .map_or_else(
                    || "no pid found!".to_string(),
                    |(pid, _)| pid.as_u32().to_string(),
                );
            ui.label(format!("gta pid: {pid}"));
        });
        ui.collapsing("app state", |ui| ui.label(format!("{self:#?}")));
    }

    fn run_timers(&mut self) {
        if self.empty_session.disabled {
            self.empty_session.countdown.count();
        } else {
            self.empty_session.countdown.reset();
        }
        if self.empty_session.interval.elapsed() >= features::empty_session::INTERVAL {
            features::empty_session::deactivate(self);
            self.empty_session.disabled = false;
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        // save any persistent state to config file
        let persistent_state = PersistentState {
            launcher: self.launch.selected,
            settings: self.settings.clone(),
        };
        persistent_state.set();
        // make sure we are not suspending game
        features::empty_session::deactivate(self);
    }
}

fn build_combo_box<E>(ui: &mut egui::Ui, current_value: &mut E, label: impl std::hash::Hash)
where
    E: strum::IntoEnumIterator + std::fmt::Display + std::cmp::PartialEq + Copy,
{
    egui::ComboBox::from_id_salt(label)
        .selected_text(current_value.to_string())
        .show_ui(ui, |ui| {
            for v in E::iter() {
                ui.selectable_value(current_value, v, v.to_string());
            }
        });
}

fn check_debug_keycombo_pressed(ctx: &egui::Context) -> bool {
    ctx.input(|i| i.modifiers.all() && i.key_pressed(egui::Key::D))
}

fn check_debug_viewport_close_button_pressed(ctx: &egui::Context) -> bool {
    ctx.input(|i| {
        i.raw
            .viewports
            .get(&egui::ViewportId::from_hash_of("debug_viewport"))
            .filter(|vp| vp.close_requested())
            .is_some()
    })
}

fn load_icon() -> egui::IconData {
    let icon = include_bytes!("../../assets/icon.png");
    let image = image::load_from_memory(icon).unwrap().into_rgba8();
    let (width, height) = image.dimensions();
    egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}

fn app_creator(
    cc: &eframe::CreationContext<'_>,
) -> Result<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>> {
    let mut app = Box::<App>::default();
    if let Some(persistent_state) = PersistentState::get() {
        app.launch.selected = persistent_state.launcher;
        app.settings = persistent_state.settings.clone();
    };
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
        "Ubuntu-Regular".to_string(),
        egui::FontData::from_static(include_bytes!("../../assets/Ubuntu-Regular.ttf")).into(),
    );
    fonts.families.insert(
        egui::FontFamily::Name("Ubuntu-Regular".into()),
        vec!["Ubuntu-Regular".to_string()],
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
            .with_icon(load_icon()),
        centered: true,
        ..Default::default()
    };
    eframe::run_native("GTA Tools", options, Box::new(app_creator)).unwrap();
}
