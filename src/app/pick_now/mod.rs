use crate::core::pick_now::{collect_record::collect_new_records, process::pn_process};
use crate::system::cache;
use crate::system::cache::error::CacheError;
use crate::system::db::{
    pn_demand::PnDemand, pn_dept_inv::PnInv, pn_past_id::PnPastID, pn_pic::PnPic,
    pn_result::PnResult, qad_item::QadItemMstr,
};
use crate::{app::error::AppError, system::state::AppState};

pub async fn handler(state: &AppState) -> Result<(), AppError> {
    let store = cache::CacheEngine::new("./cache");
    if PnResult::check_curr_data(&state).await? {
        return Err(AppError::SchedulerSkipped); // Convert string to your error type
    }
    let (past_result, pic, dmd, inv, pt_mstr) = tokio::try_join!(
        store.db_handler::<PnPastID>(&state, "pn_past_id".to_string()),
        store.db_handler::<PnPic>(&state, "pn_pic".to_string()),
        store.db_handler::<PnDemand>(&state, "pn_demand".to_string()),
        store.db_handler::<PnInv>(&state, "pn_inv".to_string()),
        store.db_handler::<QadItemMstr>(&state, "items".to_string()),
    )?;
    let result_record = collect_new_records(dmd.data, inv.data, pic.data, pt_mstr.data)?;
    let result = pn_process(result_record, past_result.data)?;
    if !result.is_empty() {
        PnResult::insert(&state, &result).await?;
    }
    Ok(())
}
