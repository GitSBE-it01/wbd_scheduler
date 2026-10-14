use time::Date;
pub mod error;
pub use error::ItemDevError;
pub mod main;
pub mod wo_parent;
use crate::{core::item_dev::wo_parent::item_dev_wo_parent_check, system::state::AppState};
pub use main::{ItemDevMstrSummary, item_dev_master_records};
pub use wo_parent::ItemDevPrntSummary;

#[derive(Debug, Clone)]
pub struct ItemDevSummary {
    pub main_sched: ItemDevMstrSummary,
    pub parent_data_sched: ItemDevPrntSummary,
}
pub async fn item_dev_handler(state: &AppState) -> Result<ItemDevSummary, ItemDevError> {
    let main_sched = item_dev_master_records(state).await?;
    let parent_data_sched = item_dev_wo_parent_check(state).await?;
    Ok(ItemDevSummary {
        main_sched,
        parent_data_sched,
    })
}
