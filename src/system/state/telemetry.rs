use time::UtcOffset;
use time::macros::format_description;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling;
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_telemetry(log_dir: &str, file_name: &str) -> WorkerGuard {
    // 1. Single append-only file (no rotation) with non-blocking I/O
    let file_appender = rolling::daily(log_dir, file_name);
    let (non_blocking_appender, guard) = tracing_appender::non_blocking(file_appender);
    let format =
        format_description!("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]");
    let timer = OffsetTime::new(
        UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC),
        format,
    );
    let console_layer = fmt::layer().pretty().with_timer(timer.clone()); // or .pretty(), .json()
    let file_layer = fmt::layer()
        .json()
        .with_ansi(false)
        .with_timer(timer)
        .flatten_event(true)
        .with_writer(non_blocking_appender);
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,wbd_scheduler=debug"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(console_layer)
        .with(file_layer)
        .init();

    guard
}
