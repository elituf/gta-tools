use crate::gui::settings::LaunchVersion;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, process::Command};
use strum::{Display, EnumIter};
use winreg::{RegKey, enums::HKEY_LOCAL_MACHINE};

#[derive(Clone, Copy, Default, Debug, Display, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
pub enum Platform {
    #[default]
    Steam,
    #[strum(to_string = "Rockstar Games")]
    Rockstar,
    #[strum(to_string = "Epic Games")]
    Epic,
}

#[derive(Debug, Default)]
pub struct Launch {
    pub selected: Platform,
}

pub fn launch(platform: &Platform, version: &LaunchVersion) {
    match platform {
        Platform::Steam => {
            let steam_url = match version {
                LaunchVersion::Enhanced => "steam://run/3240220",
                LaunchVersion::Legacy => "steam://run/271590",
            };
            open::that_detached(steam_url).unwrap();
        }
        Platform::Rockstar => {
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            let rockstar_url = match version {
                LaunchVersion::Enhanced => r"SOFTWARE\WOW6432Node\Rockstar Games\GTAV Enhanced",
                LaunchVersion::Legacy => r"SOFTWARE\WOW6432Node\Rockstar Games\Grand Theft Auto V",
            };
            let Ok(gta_v_enhanced) = hklm.open_subkey(rockstar_url) else {
                return;
            };
            let Ok(install_folder): Result<String, std::io::Error> =
                gta_v_enhanced.get_value("InstallFolder")
            else {
                return;
            };
            let mut play_gtav_path = PathBuf::from(install_folder);
            play_gtav_path.push("PlayGTAV.exe");
            // ignoring the return because if it errors that means GTA isn't installed via Rockstar
            let _ = Command::new(play_gtav_path).spawn();
        }
        Platform::Epic => {
            let epic_url = match version {
                LaunchVersion::Enhanced => {
                    "com.epicgames.launcher://apps/8769e24080ea413b8ebca3f1b8c50951?action=launch&silent=true"
                }
                LaunchVersion::Legacy => {
                    "com.epicgames.launcher://apps/9d2d0eb64d5c44529cece33fe2a46482?action=launch&silent=true"
                }
            };
            open::that_detached(epic_url).unwrap();
        }
    }
}
