use crate::app::error::AppError;
use crate::core::item_dev::{ItemDevSummary, item_dev_handler};
use crate::system::state::AppState;
use tracing::{error, info, warn};

pub async fn run_item_dev(state: &AppState) -> Result<(), AppError> {
    match item_dev_handler(state).await {
        Ok(ItemDevSummary {
            main_sched,
            parent_data_sched, // records_insert,
        }) => {
            info!(
                main_sched=?main_sched,
                parent_data_sched= ?parent_data_sched,
                "Sync finished!",
            );
        }
        Err(e) => {
            error!("Error during sync: {}", e);
        }
    }
    Ok(())
}
