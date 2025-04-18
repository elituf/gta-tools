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

impl From<Theme> for catppuccin_egui::Theme {
    fn from(val: Theme) -> Self {
        match val {
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
            Self::CatppuccinLatte => "Catppuccin Latte",
            Self::CatppuccinFrappe => "Catppuccin Frappe",
            Self::CatppuccinMacchiato => "Catppuccin Macchiato",
            Self::CatppuccinMocha => "Catppuccin Mocha",
        };
        write!(f, "{x}")
    }
}

impl Theme {
    pub fn to_catppuccin(self) -> catppuccin_egui::Theme {
        self.into()
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
