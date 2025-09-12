use crate::util::consts::path;
use std::{fs::File, io::Write};
use strum::Display;

#[derive(Clone, Copy, Display)]
pub enum LogLevel {
    Error,
    Panic,
}

pub fn log(level: LogLevel, message: &str) {
    let mut file = File::options()
        .create(true)
        .append(true)
        .open(path::APP_LOG.as_path())
        .unwrap();
    let timestamp = chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false);
    let message = format!("[{timestamp}] [{level}]\n{message}\n\n");
    file.write_all(message.as_bytes()).unwrap();
}
