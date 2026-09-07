use crate::core::pick_now::error::PnError;
use crate::system::cache;
use crate::system::cache::error::CacheError;
use crate::system::db::pn_result::PnResult;
use crate::system::state::AppState;
use std::path::Path;

pub async fn pn_cache(state: &AppState) -> Result<(), PnError> {
    let pool = state.pool();
    let depts = [
        "P1.ASSY", "PROD1.VC", "WH_ASSY", "SBE3", "SBE4", "PROD2", "PROD3", "QA", "SUBCON", "SBE4",
        "WOODWORK",
    ];
    let store = cache::CacheEngine::new("//192.168.2.103/xampp/htdocs/wbd/_cache/");
    for dpt in depts {
        let filename = format!("pn_cache_reslt__{dpt}.json");
        if let Some(cached_data) = store.get::<PnResult>(state, &filename) {
            return Ok(());
        }
        let data = PnResult::fetch_filter_dpt(state, dpt)
            .await
            .map_err(PnError::DatabaseError)?;
        let result = store
            .put::<PnResult>(state, &filename, data)
            .map_err(PnError::CacheError)?;
    }
    Ok(())
}
