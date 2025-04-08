use crate::{
    features::{
        self,
        anti_afk::AntiAfk,
        empty_session::EmptySession,
        force_close::ForceClose,
        launch::{Launch, Platform},
    },
    util::{
        consts::{ENHANCED, LEGACY},
        elevation,
    },
};
use eframe::egui;
use std::time::{Duration, Instant};
use sysinfo::System;
use windows::Win32::Foundation::HANDLE;

const THEME: catppuccin_egui::Theme = catppuccin_egui::MOCHA;

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Stage {
    #[default]
    Main,
    About,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug)]
pub struct App {
    stage: Stage,
    debug: bool,
    initialized: bool,
    elevated: bool,
    current_frame: bool,
    pub sysinfo: System,
    pub game_handle: HANDLE,
    launch: Launch,
    force_close: ForceClose,
    empty_session: EmptySession,
    anti_afk: AntiAfk,
}

impl Default for App {
    fn default() -> Self {
        Self {
            stage: Stage::default(),
            initialized: false,
            debug: false,
            elevated: elevation::is_elevated(),
            current_frame: false,
            sysinfo: System::new_all(),
            game_handle: HANDLE::default(),
            launch: Launch::default(),
            force_close: ForceClose::default(),
            empty_session: EmptySession::default(),
            anti_afk: AntiAfk::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.initialized {
            catppuccin_egui::set_theme(ctx, THEME);
            egui_extras::install_image_loaders(ctx);
            self.initialized = true;
        }
        self.run_timers();
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.stage, Stage::Main, "Main");
                ui.selectable_value(&mut self.stage, Stage::About, "About");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let button = ui
                        .add_enabled(!self.elevated, egui::Button::new("Elevate"))
                        .on_hover_text("Relaunch ourselves as administrator.")
                        .on_disabled_hover_text("We are already running elevated.");
                    if button.clicked() {
                        elevation::elevate();
                    }
                });
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| match self.stage {
                    Stage::Main => {
                        self.header(ui, "Game");
                        self.show_game(ctx, ui);
                        self.header(ui, "Session");
                        self.show_session(ctx, ui);
                        self.header(ui, "Network");
                        self.show_network(ctx, ui);
                    }
                    Stage::About => {
                        self.show_about(ctx, ui);
                    }
                });
        });
        if self.check_debug_keycombo_pressed(ctx) {
            self.debug = !self.debug;
        }
        if self.check_debug_viewport_close_button_pressed(ctx) {
            self.debug = false;
        }
        if self.debug {
            let main_rect = ctx.input(|i| i.viewport().clone().outer_rect.unwrap());
            let position = [main_rect.right(), main_rect.min.y];
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("debug_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("GTA Tools Debug")
                    .with_minimize_button(false)
                    .with_maximize_button(false)
                    .with_inner_size([256.0, 232.0])
                    .with_position(position)
                    .with_icon(load_icon()),
                |ctx, _class| {
                    if self.check_debug_keycombo_pressed(ctx) {
                        self.debug = !self.debug;
                    }
                    egui::CentralPanel::default().show(ctx, |ui| {
                        egui::ScrollArea::both()
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                self.show_debug(ctx, ui);
                            });
                    });
                },
            )
        }
        ctx.request_repaint_after(Duration::from_millis(100));
    }
}

impl App {
    fn check_debug_keycombo_pressed(&self, ctx: &egui::Context) -> bool {
        ctx.input(|i| i.modifiers.all() && i.key_pressed(egui::Key::D))
    }

    fn check_debug_viewport_close_button_pressed(&self, ctx: &egui::Context) -> bool {
        ctx.input(|i| {
            i.raw
                .viewports
                .get(&egui::ViewportId::from_hash_of("debug_viewport"))
                .filter(|vp| vp.close_requested())
                .is_some()
        })
    }

    #[allow(clippy::unused_self)]
    fn header(&self, ui: &mut egui::Ui, text: &str) {
        ui.horizontal(|ui| {
            ui.label(text);
            ui.add(egui::Separator::default().horizontal());
        });
    }

    fn show_game(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Launch").clicked() {
                features::launch::launch(&self.launch.selected);
            };
            egui::ComboBox::from_label("")
                .selected_text(self.launch.selected.to_string())
                .width(120.0)
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.launch.selected,
                        Platform::Steam,
                        Platform::Steam.to_string(),
                    );
                    ui.selectable_value(
                        &mut self.launch.selected,
                        Platform::Rockstar,
                        Platform::Rockstar.to_string(),
                    );
                    ui.selectable_value(
                        &mut self.launch.selected,
                        Platform::Epic,
                        Platform::Epic.to_string(),
                    );
                });
        });
        let force_close_button = ui.button(&self.force_close.button_text);
        if force_close_button.clicked() && !self.force_close.prompting {
            self.force_close.prompting();
            self.current_frame = true;
        };
        if self.force_close.prompting
            && self.force_close.interval.elapsed() <= Duration::from_secs(3)
        {
            if force_close_button.clicked() && !self.current_frame {
                features::force_close::activate(&mut self.sysinfo);
                self.force_close = ForceClose::default();
            }
        } else {
            self.force_close = ForceClose::default();
        }
        if self.current_frame {
            self.current_frame = false;
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
        ui.checkbox(&mut self.anti_afk.enabled, "Anti AFK")
            .on_hover_text("You should be tabbed in\nwhile this is enabled.");
        if self.anti_afk.enabled && self.anti_afk.interval.elapsed() >= features::anti_afk::INTERVAL
        {
            features::anti_afk::activate();
            self.anti_afk.interval = Instant::now();
        }
    }

    fn show_network(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::Frame::new()
            .inner_margin(egui::vec2(4.0, 4.0))
            .stroke(egui::Stroke::new(1.0, THEME.overlay1))
            .show(ui, |ui| {
                let response = ui.add_enabled_ui(self.elevated, |ui| {
                    ui.label("Game's network access");
                    ui.horizontal(|ui| {
                        if ui.button("Block").clicked() {
                            features::game_networking::block_all(&mut self.sysinfo);
                        };
                        if ui.button("Unblock").clicked() {
                            features::game_networking::unblock_all();
                        };
                    });
                });
                response
                    .response
                    .on_disabled_hover_text("This requires administrator.\nUse the Elevate button.")
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
            ui.add(egui::Image::new(egui::include_image!("../assets/icon.png")))
        });
    }

    fn show_debug(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
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
        ui.collapsing("app state", |ui| ui.label(format!("{:#?}", self)));
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

fn load_icon() -> eframe::egui::IconData {
    let icon = include_bytes!("../assets/icon.png");
    let image = image::load_from_memory(icon).unwrap().into_rgba8();
    let (width, height) = image.dimensions();
    eframe::egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}

pub fn run() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(false)
            .with_maximize_button(false)
            .with_inner_size([256.0, 232.0])
            .with_icon(load_icon()),
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "GTA Tools",
        options,
        Box::new(|_cc| Ok(Box::<App>::default())),
    )
    .unwrap();
}
