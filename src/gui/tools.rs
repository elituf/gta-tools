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

pub fn debug_keycombo_pressed(ctx: &egui::Context) -> bool {
    ctx.input(|i| i.modifiers.all() && i.key_pressed(egui::Key::D))
}

pub fn debug_viewport_close_pressed(ctx: &egui::Context) -> bool {
    ctx.input(|i| {
        i.raw
            .viewports
            .get(&egui::ViewportId::from_hash_of("debug_viewport"))
            .filter(|vp| vp.close_requested())
            .is_some()
    })
}
