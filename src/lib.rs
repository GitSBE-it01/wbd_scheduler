// general modules
pub mod app_config;
pub mod app_error;
pub mod app_state;
pub mod app_utilities;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};
pub fn init_tracing() {
    let filter_layer = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(
            fmt::layer()
                // .with_thread_ids(true) // Shows thread ID (helpful for async tokio tasks)
                // .with_thread_names(true)
                .with_target(true), // .with_line_number(true),
        )
        .init();
}

// specialized services
pub mod pick_now;
