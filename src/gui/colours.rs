use crate::features::game_networking::BlockedStatus;
use eframe::egui;

pub const RED: egui::Color32 = egui::Color32::from_rgb(255, 96, 96);
pub const YELLOW: egui::Color32 = egui::Color32::from_rgb(255, 255, 96);
pub const GREEN: egui::Color32 = egui::Color32::from_rgb(96, 255, 96);

impl From<BlockedStatus> for egui::Color32 {
    fn from(value: BlockedStatus) -> Self {
        match value {
            BlockedStatus::Blocked => RED,
            BlockedStatus::Failed => YELLOW,
            BlockedStatus::Unblocked => GREEN,
        }
    }
}

impl BlockedStatus {
    pub fn to_color32(self) -> egui::Color32 {
        self.into()
    }
}
