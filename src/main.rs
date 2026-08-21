use std::process::ExitCode;
use tracing::error;
use wbd_scheduler::cli::main_handlers::main_run;
use wbd_scheduler::core::state::telemetry;

#[tokio::main]
async fn main() -> ExitCode {
    let _guard = telemetry::init_telemetry("./logs", "scheduler.log");
    if let Err(err) = main_run().await {
        error!(error = ?err, "Application exited with error");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
