use eframe::egui;

pub trait UiExt {
    fn add_sized_left_aligned(
        &mut self,
        max_size: impl Into<egui::Vec2>,
        widget: impl egui::Widget,
    ) -> egui::Response;
}

impl UiExt for egui::Ui {
    fn add_sized_left_aligned(
        &mut self,
        max_size: impl Into<egui::Vec2>,
        widget: impl egui::Widget,
    ) -> egui::Response {
        let layout = egui::Layout::top_down_justified(egui::Align::LEFT);
        self.allocate_ui_with_layout(max_size.into(), layout, |ui| ui.add(widget))
            .inner
    }
}
