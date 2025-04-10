use crate::{features::launch::Platform, gui::settings::Settings};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    sync::LazyLock,
};

static CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    dirs::config_local_dir()
        .unwrap()
        .join("GTA Tools")
        .join("config.json")
});

#[derive(Serialize, Deserialize)]
pub struct PersistentState {
    pub launcher: Platform,
    pub settings: Settings,
}

impl PersistentState {
    pub fn get() -> Option<Self> {
        fs::read_to_string(CONFIG_PATH.as_path())
            .ok()
            .and_then(|config| serde_json::from_str::<PersistentState>(&config).ok())
    }

    pub fn set(&self) {
        let config_path = CONFIG_PATH.as_path();
        let config_path_parent = config_path.parent().unwrap();
        if !config_path_parent.exists() {
            fs::create_dir(config_path_parent).unwrap();
        }
        let mut config_file = File::create(config_path).unwrap();
        let json = serde_json::to_string_pretty(&self).unwrap();
        config_file.write_all(json.as_bytes()).unwrap();
    }
}
