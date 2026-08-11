use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_telemetry(log_dir: &str, file_name: &str) -> WorkerGuard {
    // 1. Single append-only file (no rotation) with non-blocking I/O
    let file_appender = rolling::never(log_dir, file_name);
    let (non_blocking_appender, guard) = tracing_appender::non_blocking(file_appender);

    // 2. Output Layer: Console (Human-readable during development)
    let console_layer = fmt::layer().compact(); // or .pretty(), .json()

    // 3. Output Layer: File (Structured JSON, no color ANSI codes)
    let file_layer = fmt::layer()
        .json()
        .with_ansi(false)
        .with_writer(non_blocking_appender);

    // 4. Environment Filter (e.g. RUST_LOG=info,my_crate=debug)
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,wbd_scheduler=debug"));

    // 5. Combine layers into a single subscriber pipeline
    tracing_subscriber::registry()
        .with(env_filter)
        .with(console_layer)
        .with(file_layer)
        .init();

    guard
}
