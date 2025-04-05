use crate::{
    features::{
        self,
        anti_afk::AntiAfk,
        empty_session::EmptySession,
        force_close::ForceClose,
        launch::{Launch, Platform},
    },
    util::elevate,
};
use eframe::egui;
use std::time::{Duration, Instant};
use sysinfo::System;
use windows::Win32::Foundation::HANDLE;

pub struct App {
    pub current_frame: bool,
    pub sysinfo: System,
    pub game_handle: HANDLE,
    pub launch: Launch,
    pub force_close: ForceClose,
    pub empty_session: EmptySession,
    pub anti_afk: AntiAfk,
}

impl Default for App {
    fn default() -> Self {
        Self {
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
        catppuccin_egui::set_theme(ctx, catppuccin_egui::MOCHA);
        ctx.request_repaint_after(Duration::from_millis(100));
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Game");
                ui.add(egui::Separator::default().horizontal());
            });
            ui.horizontal(|ui| {
                if ui.button("Launch").clicked() {
                    features::launch::launch(&self.launch.selected);
                };
                egui::ComboBox::from_label("")
                    .selected_text(self.launch.selected.to_string())
                    .width(125.0)
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
                self.force_close.button_text = "Are you sure?".to_string();
                self.force_close.prompting = true;
                self.force_close.interval = Instant::now();
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
            ui.horizontal(|ui| {
                ui.label("Session");
                ui.add(egui::Separator::default().horizontal());
            });
            ui.add_enabled_ui(!self.empty_session.disabled, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Empty current session").clicked() {
                        self.empty_session.interval = Instant::now();
                        self.empty_session.disabled = true;
                        features::empty_session::activate(self);
                    }
                    if self.empty_session.disabled {
                        self.empty_session.countdown.count();
                    } else {
                        self.empty_session.countdown.reset();
                    }
                    if self.empty_session.interval.elapsed() >= features::empty_session::INTERVAL {
                        features::empty_session::deactivate(self);
                        self.empty_session.disabled = false;
                    }
                    ui.label(&self.empty_session.countdown.i_string);
                });
            });
            ui.checkbox(&mut self.anti_afk.enabled, "Anti AFK")
                .on_hover_text("You should be tabbed in\nwhile this is enabled.");
            if self.anti_afk.enabled
                && self.anti_afk.interval.elapsed() >= features::anti_afk::INTERVAL
            {
                features::anti_afk::activate();
                self.anti_afk.interval = Instant::now();
            }
            // ui.horizontal(|ui| {
            //     ui.label("Network");
            //     ui.add(egui::Separator::default().horizontal());
            // });
            // if ui.button("Elevate").clicked() {
            //     elevate::elevate();
            // }
            // ui.checkbox(&mut false, "Block connections to Rockstar");
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.horizontal(|ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.label("with love from ");
                        ui.hyperlink_to("futile", "http://futile.eu");
                    });
                    ui.separator();
                    ui.label(format!(
                        "v{} {}",
                        env!("CARGO_PKG_VERSION"),
                        if cfg!(debug_assertions) { "(dev)" } else { "" }
                    ));
                });
                // ui.separator();
            });
        });
    }
}

fn load_icon() -> eframe::egui::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let icon = include_bytes!("../assets/icon.png");
        let image = image::load_from_memory(icon).unwrap().into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    eframe::egui::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

pub fn run() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(false)
            .with_maximize_button(false)
            .with_inner_size([250.0, 225.0])
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
