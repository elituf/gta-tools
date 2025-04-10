use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::EnumIter;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
pub enum Theme {
    CatppuccinLatte,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    CatppuccinMocha,
}

impl Into<catppuccin_egui::Theme> for Theme {
    fn into(self) -> catppuccin_egui::Theme {
        match self {
            Theme::CatppuccinLatte => catppuccin_egui::LATTE,
            Theme::CatppuccinFrappe => catppuccin_egui::FRAPPE,
            Theme::CatppuccinMacchiato => catppuccin_egui::MACCHIATO,
            Theme::CatppuccinMocha => catppuccin_egui::MOCHA,
        }
    }
}

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            Theme::CatppuccinLatte => "Catppuccin Latte",
            Theme::CatppuccinFrappe => "Catppuccin Frappe",
            Theme::CatppuccinMacchiato => "Catppuccin Macchiato",
            Theme::CatppuccinMocha => "Catppuccin Mocha",
        };
        write!(f, "{x}")
    }
}

impl Theme {
    pub fn to_catppuccin(&self) -> catppuccin_egui::Theme {
        (*self).into()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub theme: Theme,
    pub start_elevated: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: Theme::CatppuccinMocha,
            start_elevated: false,
        }
    }
}
