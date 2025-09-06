use crate::{features::launch::Platform, gui::settings::Settings, util::consts::path};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Write,
};

#[derive(Serialize, Deserialize)]
pub struct PersistentState {
    pub launcher: Platform,
    pub settings: Settings,
}

impl PersistentState {
    pub fn get() -> Option<Self> {
        fs::read_to_string(path::APP_CONFIG.as_path())
            .ok()
            .and_then(|config| serde_json::from_str::<Self>(&config).ok())
    }

    pub fn set(&self) {
        let mut config_file = File::create(path::APP_CONFIG.as_path()).unwrap();
        let json = serde_json::to_string_pretty(&self).unwrap();
        config_file.write_all(json.as_bytes()).unwrap();
    }
}
