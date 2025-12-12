use crate::{
    gui::{
        app::{App, WINDOW_SIZE},
        tools,
    },
    util::consts::game::{EXE_ENHANCED, EXE_LEGACY},
};
use eframe::egui;

impl App {
    fn add_debug_viewport_contents(&mut self, ui: &mut egui::Ui) {
        ui.collapsing("misc", |ui| {
            ui.scope(|ui| {
                use windows::Win32::UI::WindowsAndMessaging::{
                    GetForegroundWindow, GetWindowTextW,
                };
                let current_title = {
                    let mut buffer = [0; 512];
                    let hwnd = unsafe { GetForegroundWindow() };
                    let length = unsafe { GetWindowTextW(hwnd, &mut buffer) };
                    String::from_utf16_lossy(&buffer[..length as usize])
                };
                ui.label(format!("focused: \"{current_title}\""));
            });
            ui.horizontal(|ui| {
                ui.label("blocked");
                egui::ComboBox::from_id_salt("blocked")
                    .selected_text(self.game_networking.blocked.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.game_networking.blocked, true, "true");
                        ui.selectable_value(&mut self.game_networking.blocked, false, "false");
                    });
            });
            if ui.add(egui::Button::new("force refresh theme")).clicked() {
                catppuccin_egui::set_theme(ui.ctx(), self.settings.theme.into());
            }
            if ui.button("panic button").clicked() {
                panic!("this is the panic button");
            }
        });
        ui.collapsing("anti afk", |ui| {
            ui.label(format!(
                "timer: {}",
                self.anti_afk.interval.elapsed().as_secs()
            ));
            ui.label(format!("can activate: {}", self.anti_afk.can_activate()));
        });
        ui.collapsing("system info", |ui| {
            if ui.button("refresh").clicked() {
                self.system_info.refresh();
            }
            let pid = self
                .system_info
                .processes()
                .iter()
                .find(|p| p.name() == EXE_ENHANCED || p.name() == EXE_LEGACY)
                .map_or_else(|| "no pid found!".to_owned(), |p| p.pid().to_string());
            ui.label(format!("gta pid: {pid}"));
        });
        ui.collapsing("app state", |ui| ui.label(format!("{self:#?}")));
    }

    pub fn show_debug_viewport(&mut self, ctx: &egui::Context) {
        let main = ctx.input(|i| i.viewport().outer_rect.unwrap_or(egui::Rect::EVERYTHING));
        let builder = egui::ViewportBuilder::default()
            .with_title("GTA Tools Debug")
            .with_minimize_button(false)
            .with_maximize_button(false)
            .with_inner_size(WINDOW_SIZE)
            .with_position([main.right() - 12.0, main.min.y])
            .with_icon(tools::load_icon());
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("debug_viewport"),
            builder,
            |ctx, _class| {
                if tools::debug_keycombo_pressed(ctx) {
                    self.flags.debug = !self.flags.debug;
                }
                egui::CentralPanel::default().show(ctx, |ui| {
                    egui::ScrollArea::both()
                        .auto_shrink([false, true])
                        .show(ui, |ui| self.add_debug_viewport_contents(ui));
                });
            },
        );
    }
}
