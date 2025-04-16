use std::{path::PathBuf, sync::LazyLock};

pub const ENHANCED: &str = "GTA5_Enhanced.exe";
pub const LEGACY: &str = "GTA5.exe";

pub const GTA_WINDOW_TITLE: &str = "Grand Theft Auto V";

pub static APP_STORAGE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| dirs::config_local_dir().unwrap().join("GTA Tools"));
