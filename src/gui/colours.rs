use crate::features::game_networking::BlockedStatus;
use eframe::egui;

pub const RED: egui::Color32 = egui::Color32::from_rgb(255, 128, 128);
pub const YELLOW: egui::Color32 = egui::Color32::from_rgb(255, 255, 128);
pub const GREEN: egui::Color32 = egui::Color32::from_rgb(128, 255, 128);

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
