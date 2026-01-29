use std::{path::PathBuf, sync::OnceLock};

use directories::ProjectDirs;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

static LOG_GUARD: OnceLock<WorkerGuard> = OnceLock::new();

fn default_log_dir() -> PathBuf {
    let proj = ProjectDirs::from("io", "", "redice").expect("ProjectDirs failed");
    proj.data_local_dir().join("logs")
}

pub fn init_logging() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout)
        .with_ansi(true)
        .with_line_number(true);

    let registry = tracing_subscriber::registry().with(filter);

    if cfg!(debug_assertions) {
        registry.with(stdout_layer).init();
    } else {
        let log_dir = default_log_dir();
        std::fs::create_dir_all(&log_dir).ok();
        let file_appender = tracing_appender::rolling::daily(log_dir, "redice.log");
        let (file_writer, guard) = tracing_appender::non_blocking(file_appender);

        let _ = LOG_GUARD.set(guard);

        let file_layer = tracing_subscriber::fmt::layer()
            .with_writer(file_writer)
            .with_ansi(false)
            .with_file(true)
            .with_line_number(true);

        registry.with(stdout_layer).with(file_layer).init();
    }
}
