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

pub fn launch(platform: &Platform) {
    match platform {
        Platform::Steam => {
            let _ = open::that_detached("steam://run/3240220");
        }
        Platform::Rockstar => {
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            let Ok(gta_v_enhanced) =
                hklm.open_subkey(r"SOFTWARE\WOW6432Node\Rockstar Games\GTAV Enhanced")
            else {
                return;
            };
            let Ok(install_folder): Result<String, std::io::Error> =
                gta_v_enhanced.get_value("InstallFolder")
            else {
                return;
            };
            let mut play_gtav_path = PathBuf::from(install_folder);
            play_gtav_path.push("PlayGTAV.exe");
            let _ = Command::new(play_gtav_path).spawn();
        }
        Platform::Epic => {
            let _ = open::that_detached(
                "com.epicgames.launcher://apps/331226ba7c944720baa99103cb1fe80c?action=launch&silent=true",
            );
        }
    }
}
