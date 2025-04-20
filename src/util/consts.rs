use std::{path::PathBuf, sync::LazyLock};

pub static APP_STORAGE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| dirs::config_local_dir().unwrap().join("GTA Tools"));

pub mod game {
    pub const EXE_ENHANCED: &str = "GTA5_Enhanced.exe";
    pub const EXE_LEGACY: &str = "GTA5.exe";
    pub const WINDOW_TITLE: &str = "Grand Theft Auto V";
}
