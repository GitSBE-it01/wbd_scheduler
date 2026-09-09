pub mod collect_record;
pub mod error;
pub mod model;
pub mod pn_cache;
pub mod process;
use crate::system::cache;
use crate::system::db::{
    pn_demand::PnDemand, pn_dept_inv::PnInv, pn_past_id::PnPastID, pn_pic::PnPic,
    pn_result::PnResult, qad_item::QadItemMstr,
};
use crate::system::state::AppState;
use collect_record::collect_new_records;
use error::PnError;
use model::{PnOutcome, PnSummary};
use pn_cache::PnCache;
use process::pn_process;

pub async fn run_pick_now(state: &AppState) -> Result<PnOutcome, PnError> {
    let store = cache::CacheEngine::new("./cache");
    if PnResult::check_curr_data(state).await? {
        return Ok(PnOutcome::Skipped {
            reason: "Pick Now already run".to_string(),
        });
    }
    let (past_result, pic, dmd, inv, pt_mstr) = tokio::try_join!(
        store.db_handler::<PnPastID>(state, "pn_past_id".to_string()),
        store.db_handler::<PnPic>(state, "pn_pic".to_string()),
        store.db_handler::<PnDemand>(state, "pn_demand".to_string()),
        store.db_handler::<PnInv>(state, "pn_inv".to_string()),
        store.db_handler::<QadItemMstr>(state, "items".to_string()),
    )?;
    let result_record = collect_new_records(dmd.data, inv.data, pic.data, pt_mstr.data)?;
    let final_result = pn_process(result_record, past_result.data)?;
    let comp_count = final_result.len();
    let inst_count: usize = if !final_result.is_empty() {
        let cnt = PnResult::insert(state, &final_result).await?;
        PnCache(state).await?;
        cnt.try_into().unwrap()
    } else {
        0
    };
    Ok(PnOutcome::Completed(PnSummary {
        computed_item: comp_count,
        items_inserted: inst_count,
    }))
}
