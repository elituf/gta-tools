use eframe::egui;

pub trait UiExt {
    fn header(&mut self, text: &str);
    fn build_menu<E>(&mut self, current_value: &mut E)
    where
        E: strum::IntoEnumIterator + std::fmt::Display + std::cmp::PartialEq + Copy;
    fn create_indicator_dot(&mut self, colour: impl Into<egui::Color32>) -> egui::Response;
}

impl UiExt for egui::Ui {
    fn header(&mut self, text: &str) {
        self.horizontal(|ui| {
            ui.label(egui::RichText::new(text).font(egui::FontId::new(
                12.5,
                egui::FontFamily::Name("Inter 18pt Regular".into()),
            )));
            ui.add(egui::Separator::default().horizontal());
        });
    }

    fn build_menu<E>(&mut self, current_value: &mut E)
    where
        E: strum::IntoEnumIterator + std::fmt::Display + std::cmp::PartialEq + Copy,
    {
        E::iter().for_each(|variant| {
            self.selectable_value(current_value, variant, variant.to_string());
        });
    }

    fn create_indicator_dot(&mut self, colour: impl Into<egui::Color32>) -> egui::Response {
        self.add(
            egui::Image::new(egui::include_image!("../../assets/circle.svg"))
                .max_size([4.0, 4.0].into())
                .tint(colour),
        )
    }
}
