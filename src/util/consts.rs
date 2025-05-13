pub mod path {
    use std::{env, path::PathBuf, sync::LazyLock};
    pub static APP_STORAGE: LazyLock<PathBuf> = LazyLock::new(|| {
        PathBuf::from(env::var("LOCALAPPDATA").unwrap_or_else(|_| String::from(".")))
            .join("GTA Tools")
    });
    pub static APP_CONFIG: LazyLock<PathBuf> = LazyLock::new(|| APP_STORAGE.join("config.json"));
    pub static APP_ERROR: LazyLock<PathBuf> = LazyLock::new(|| APP_STORAGE.join("error.log"));
}

pub mod game {
    pub const EXE_ENHANCED: &str = "GTA5_Enhanced.exe";
    pub const EXE_LEGACY: &str = "GTA5.exe";
    pub const WINDOW_TITLE: &str = "Grand Theft Auto V";
}
