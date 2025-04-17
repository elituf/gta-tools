use eframe::egui;

pub fn load_icon() -> egui::IconData {
    let icon = include_bytes!("../../assets/icon.png");
    let image = image::load_from_memory(icon).unwrap().into_rgba8();
    let (width, height) = image.dimensions();
    egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}

pub fn header(ui: &mut egui::Ui, text: &str) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(text).font(egui::FontId::new(
            12.5,
            egui::FontFamily::Name("Ubuntu-Regular".into()),
        )));
        ui.add(egui::Separator::default().horizontal());
    });
}

pub fn build_combo_box<E>(ui: &mut egui::Ui, current_value: &mut E, label: impl std::hash::Hash)
where
    E: strum::IntoEnumIterator + std::fmt::Display + std::cmp::PartialEq + Copy,
{
    egui::ComboBox::from_id_salt(label)
        .selected_text(current_value.to_string())
        .show_ui(ui, |ui| {
            E::iter().for_each(|v| {
                ui.selectable_value(current_value, v, v.to_string());
            });
        });
}

pub fn check_debug_keycombo_pressed(ctx: &egui::Context) -> bool {
    ctx.input(|i| i.modifiers.all() && i.key_pressed(egui::Key::D))
}

pub fn check_debug_viewport_close_button_pressed(ctx: &egui::Context) -> bool {
    ctx.input(|i| {
        i.raw
            .viewports
            .get(&egui::ViewportId::from_hash_of("debug_viewport"))
            .filter(|vp| vp.close_requested())
            .is_some()
    })
}
