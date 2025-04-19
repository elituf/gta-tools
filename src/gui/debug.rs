use crate::{
    gui::{
        app::{App, WINDOW_SIZE},
        tools,
    },
    util::consts::{
        APP_STORAGE_PATH,
        game::{EXE_ENHANCED, EXE_LEGACY},
    },
};
use eframe::egui;

impl App {
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
                        .show(ui, |ui| {
                            ui.collapsing("misc", |ui| {
                                if ui.button("open storage path").clicked() {
                                    open::that_detached(APP_STORAGE_PATH.as_path()).unwrap();
                                }
                                ui.checkbox(
                                    &mut self.meta.newer_version_available,
                                    "spoof new version available",
                                )
                                .on_hover_text("(this could already be checked if\nthere actually IS a new version)");
                                ui.scope(|ui| {
                                    use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW};
                                    let mut buffer = [0; 512];
                                    let current_title = unsafe {
                                        let hwnd = GetForegroundWindow();
                                        let length = GetWindowTextW(hwnd, &mut buffer);
                                        String::from_utf16_lossy(&buffer[..length as usize])
                                    };
                                    ui.label(format!("focused: \"{current_title}\""));
                                });
                            });
                            ui.collapsing("anti afk", |ui| {
                                ui.label(format!(
                                    "timer: {}",
                                    self.anti_afk.interval.elapsed().as_secs()
                                ));
                                ui.label(format!(
                                    "can activate: {}",
                                    self.anti_afk.can_activate()
                                ));
                            });
                            ui.collapsing("sysinfo", |ui| {
                                if ui.button("refresh all").clicked() {
                                    self.sysinfo.refresh_all();
                                }
                                let pid = self
                                    .sysinfo
                                    .processes()
                                    .iter()
                                    .find(|(_, p)| p.name() == EXE_ENHANCED || p.name() == EXE_LEGACY)
                                    .map_or_else(
                                        || "no pid found!".to_owned(),
                                        |(pid, _)| pid.as_u32().to_string(),
                                    );
                                ui.label(format!("gta pid: {pid}"));
                            });
                            ui.collapsing("app state", |ui| ui.label(format!("{self:#?}")));
                        });
                });
            },
        );
    }
}
