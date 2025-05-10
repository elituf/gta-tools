use crate::util::win;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(Clone, Copy, Debug, Default, Display, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
pub enum Theme {
    #[default]
    #[strum(to_string = "Auto")]
    Auto,
    #[strum(to_string = "Catppuccin Latte")]
    CatppuccinLatte,
    #[strum(to_string = "Catppuccin Frappe")]
    CatppuccinFrappe,
    #[strum(to_string = "Catppuccin Macchiato")]
    CatppuccinMacchiato,
    #[strum(to_string = "Catppuccin Mocha")]
    CatppuccinMocha,
}

impl From<Theme> for catppuccin_egui::Theme {
    fn from(val: Theme) -> Self {
        match val {
            Theme::Auto => {
                if win::is_system_theme_dark() {
                    catppuccin_egui::MOCHA
                } else {
                    catppuccin_egui::LATTE
                }
            }
            Theme::CatppuccinLatte => catppuccin_egui::LATTE,
            Theme::CatppuccinFrappe => catppuccin_egui::FRAPPE,
            Theme::CatppuccinMacchiato => catppuccin_egui::MACCHIATO,
            Theme::CatppuccinMocha => catppuccin_egui::MOCHA,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Settings {
    pub theme: Theme,
    pub start_elevated: bool,
}
