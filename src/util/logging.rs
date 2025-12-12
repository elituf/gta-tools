use std::{
    fs::File,
    io::Write,
    sync::{LazyLock, Mutex},
    time::SystemTime,
};

pub struct Logger {
    file: Mutex<File>,
}

static LOGGER: LazyLock<Logger> = LazyLock::new(|| Logger {
    file: File::options()
        .create(true)
        .append(true)
        .open(crate::util::consts::path::APP_LOG.as_path())
        .unwrap()
        .into(),
});

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn flush(&self) {}

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            Self::log_to_stderr(record);
            self.log_to_file(record);
        }
    }
}

impl Logger {
    pub fn init(level: log::LevelFilter) {
        log::set_logger(&*LOGGER)
            .map(|()| log::set_max_level(level))
            .unwrap();
    }

    fn log_to_file(&self, record: &log::Record) {
        let mut file = self.file.lock().unwrap();
        writeln!(
            file,
            "[{}][{}] {}",
            humantime::format_rfc3339_seconds(SystemTime::now()),
            record.level(),
            record.args()
        )
        .unwrap();
        file.flush().unwrap();
    }

    fn log_to_stderr(record: &log::Record) {
        eprintln!(
            "[{}][{}] {}",
            humantime::format_rfc3339_seconds(SystemTime::now()),
            record.level(),
            record.args()
        );
    }
}
