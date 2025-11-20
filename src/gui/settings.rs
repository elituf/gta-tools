use crate::util::win;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

pub const ROCKSTAR_SAVE_SERVER: &str = "192.81.241.171";

#[derive(Clone, Copy, Debug, Default, Display, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
pub enum Theme {
    #[default]
    #[strum(to_string = "Auto")]
    Auto,
    #[strum(to_string = "Latte")]
    Latte,
    #[strum(to_string = "Frappe")]
    Frappe,
    #[strum(to_string = "Macchiato")]
    Macchiato,
    #[strum(to_string = "Mocha")]
    Mocha,
}

impl From<Theme> for catppuccin_egui::Theme {
    fn from(val: Theme) -> Self {
        match val {
            Theme::Auto => {
                if win::is_system_theme_light() {
                    catppuccin_egui::LATTE
                } else {
                    catppuccin_egui::MOCHA
                }
            }
            Theme::Latte => catppuccin_egui::LATTE,
            Theme::Frappe => catppuccin_egui::FRAPPE,
            Theme::Macchiato => catppuccin_egui::MACCHIATO,
            Theme::Mocha => catppuccin_egui::MOCHA,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Display, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
pub enum LaunchVersion {
    #[default]
    Enhanced,
    Legacy,
}

#[derive(Clone, Copy, Debug, Default, Display, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
pub enum BlockMethod {
    #[default]
    #[strum(to_string = "Entire game")]
    EntireGame,
    #[strum(to_string = "Save server")]
    SaveServer,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub start_elevated: bool,
    pub theme: Theme,
    pub launch_version: LaunchVersion,
    pub save_server_ip: String,
    pub block_method: BlockMethod,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            start_elevated: false,
            theme: Theme::default(),
            launch_version: LaunchVersion::default(),
            block_method: BlockMethod::default(),
            save_server_ip: String::from(ROCKSTAR_SAVE_SERVER),
        }
    }
}
