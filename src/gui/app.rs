use crate::{
    features,
    gui::{settings::Settings, tools, ui_ext::UiExt},
    util::{
        consts::{colours, game::WINDOW_TITLE},
        persistent_state::PersistentState,
        system_info::SystemInfo,
        win,
    },
};
use eframe::egui;
use std::time::{Duration, Instant};
use strum::{Display, EnumIter};

pub const WINDOW_SIZE: [f32; 2] = [240.0, 240.0];

#[derive(Clone, Copy, Debug, Default, Display, PartialEq, Eq, EnumIter)]
enum Stage {
    #[default]
    Main,
    Settings,
    About,
}

#[derive(Debug)]
pub struct Flags {
    pub elevated: bool,
    pub debug: bool,
    closing: bool,
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            elevated: win::is_elevated(),
            debug: false,
            closing: false,
        }
    }
}

#[derive(Debug, Default)]
pub struct App {
    pub settings: Settings,
    stage: Stage,
    pub flags: Flags,
    pub system_info: SystemInfo,
    game_handle: windows::Win32::Foundation::HANDLE,
    pub anti_afk: features::anti_afk::AntiAfk,
    empty_session: features::empty_session::EmptySession,
    force_close: features::force_close::ForceClose,
    pub game_networking: features::game_networking::GameNetworking,
    pub launch: features::launch::Launch,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(Duration::from_millis(100));
        self.empty_session.run_timers(&mut self.game_handle);
        egui::TopBottomPanel::bottom("bottom_panel")
            .exact_height(25.0)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.build_menu(&mut self.stage);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let button = ui
                            .add_enabled(!self.flags.elevated, egui::Button::new("Elevate"))
                            .on_hover_text("Relaunch ourselves as administrator.")
                            .on_disabled_hover_text("We are already running elevated.");
                        if button.clicked() {
                            win::elevate(win::ElevationExitMethod::Gentle(&mut self.flags.closing));
                        }
                    });
                });
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, true])
                .show(ui, |ui| match self.stage {
                    Stage::Main => self.show_main_stage(ctx, ui),
                    Stage::Settings => self.show_settings_stage(ctx, ui),
                    Stage::About => self.show_about_stage(ctx, ui),
                });
        });
        if tools::debug_keycombo_pressed(ctx) || tools::debug_viewport_close_pressed(ctx) {
            self.flags.debug = !self.flags.debug;
        }
        if self.flags.debug {
            self.show_debug_viewport(ctx);
        }
        if self.flags.closing {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}

impl App {
    fn show_game_section(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.header("Game");
        ui.horizontal(|ui| {
            if ui.button("Launch").clicked() {
                features::launch::launch(&self.launch.selected, &self.settings.launch_version);
            }
            egui::ComboBox::from_id_salt("Launch")
                .selected_text(self.launch.selected.to_string())
                .width(120.0)
                .show_ui(ui, |ui| {
                    ui.build_menu(&mut self.launch.selected);
                });
        });
        let force_close_button = ui.add_sized(
            [104.0, 0.0],
            egui::Button::new(&self.force_close.button_text),
        );
        self.force_close
            .prompt(force_close_button.clicked(), &mut self.system_info);
    }

    fn show_session_section(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.header("Session");
        ui.add_enabled_ui(!self.empty_session.disabled, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Empty current session").clicked()
                    && features::empty_session::activate(
                        &mut self.game_handle,
                        &mut self.system_info,
                    )
                {
                    self.empty_session.interval = Instant::now();
                    self.empty_session.disabled = true;
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
                    ui.label(if win::is_window_focused(WINDOW_TITLE) {
                        "GTA is focused."
                    } else {
                        "GTA is not focused!"
                    })
                });
            }
        });
        if self.anti_afk.can_activate() && self.anti_afk.should_activate() {
            self.anti_afk.activate();
        }
    }

    fn show_network_section(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.header("Network");
        egui::Frame::new()
            .outer_margin(egui::vec2(0.0, -2.0))
            .show(ui, |ui| {
                let response = ui.add_enabled_ui(self.flags.elevated, |ui| {
                    let label = ui.horizontal(|ui| {
                        let label = ui.label("Game's network access");
                        ui.add_space(1.0);
                        ui.create_indicator_dot(self.game_networking.blocked_status)
                            .on_hover_text(
                                "This turns yellow if GTA Tools\ncannot find your game.",
                            );
                        self.game_networking.if_failed_return_to_boolean();
                        label
                    });
                    ui.horizontal(|ui| {
                        let available_width = label.inner.rect.width();
                        let spacing = ui.spacing().item_spacing.x;
                        let button_width = (available_width - spacing) / 2.0;
                        if ui
                            .add_sized([button_width, 18.0], egui::Button::new("Block"))
                            .clicked()
                        {
                            self.game_networking.block_all(&mut self.system_info);
                        }
                        if ui
                            .add_sized([button_width, 18.0], egui::Button::new("Unblock"))
                            .clicked()
                        {
                            self.game_networking.unblock_all();
                        }
                    });
                });
                response.response.on_disabled_hover_text(
                    "This requires administrator.\nUse the Elevate button.",
                );
            });
    }

    fn show_main_stage(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        self.show_game_section(ctx, ui);
        self.show_session_section(ctx, ui);
        self.show_network_section(ctx, ui);
    }

    fn show_settings_stage(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Launch version");
            egui::ComboBox::from_id_salt("Launch version")
                .selected_text(self.settings.launch_version.to_string())
                .show_ui(ui, |ui| {
                    ui.build_menu(&mut self.settings.launch_version);
                });
        });
        ui.horizontal(|ui| {
            let selection = self.settings.theme;
            ui.label("Theme");
            egui::ComboBox::from_id_salt("Theme")
                .selected_text(self.settings.theme.to_string())
                .show_ui(ui, |ui| {
                    ui.build_menu(&mut self.settings.theme);
                });
            if selection != self.settings.theme {
                catppuccin_egui::set_theme(ctx, self.settings.theme.into());
            }
        });
        ui.checkbox(&mut self.settings.start_elevated, "Always start elevated");
    }

    fn show_about_stage(&self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            ui.horizontal(|ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("with ");
                    ui.scope(|ui| {
                        ui.style_mut().visuals.override_text_color = Some(colours::RED);
                        ui.label("❤");
                    });
                    ui.label(" from ");
                    ui.hyperlink_to("futile", "https://futile.eu");
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.style_mut().spacing.button_padding = egui::Vec2::new(4.0, 0.0);
                    if ui.button("").on_hover_text("View source code").clicked() {
                        open::that("https://github.com/elituf/gta-tools").unwrap();
                    }
                    if cfg!(debug_assertions) {
                        ui.label("(dev)");
                    }
                    ui.label(format!("v{}", env!("CARGO_PKG_VERSION")))
                        .on_hover_text(env!("LATEST_GIT_COMMIT_HASH"));
                });
            });
            ui.add(egui::Image::new(egui::include_image!(
                "../../assets/icon.png"
            )));
        });
    }
}

impl Drop for App {
    fn drop(&mut self) {
        // save any persistent state to config file
        PersistentState {
            launcher: self.launch.selected,
            anti_afk_enabled: self.anti_afk.enabled,
            settings: self.settings.clone(),
        }
        .set();
        // make sure we are not suspending game
        features::empty_session::deactivate(&mut self.game_handle);
    }
}
