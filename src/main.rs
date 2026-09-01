use std::process::ExitCode;
use tracing::error;
use wbd_scheduler::app::main_handler;
use wbd_scheduler::system::state::telemetry;

#[tokio::main]
async fn main() -> ExitCode {
    let _guard = telemetry::init_telemetry("./logs", "scheduler.log");
    if let Err(err) = main_handler().await {
        error!(error = ?err, "Application exited with error");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
