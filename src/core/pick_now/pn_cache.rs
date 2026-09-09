use crate::core::pick_now::error::PnError;
use crate::system::cache;
use crate::system::cache::error::CacheError;
use crate::system::db::pn_result::PnResult;
use crate::system::state::AppState;
use std::path::Path;
use tracing::info;

#[derive(Debug, Clone)]
pub struct PnCacheLog {
    pub department: String,
    pub item_count: usize,
    pub from_cache: bool,
}
pub async fn PnCache(state: &AppState) -> Result<Vec<PnCacheLog>, PnError> {
    let pool = state.pool();
    let depts = [
        "P1.ASSY", "PROD1.VC", "WH_ASSY", "SBE3", "SBE4", "PROD2", "PROD3", "QA", "SUBCON", "SBE4",
        "WOODWORK",
    ];
    let store = cache::CacheEngine::new("//192.168.2.103/xampp/htdocs/wbd/_cache/");
    let mut logs = Vec::new();
    for dpt in depts {
        let filename = format!("pn_cache_reslt__{dpt}");
        if let Some(cached_data) = store.get::<PnResult>(state, &filename) {
            let count = cached_data.data.len();

            info!(
                department = dpt,
                count = count,
                source = "cache",
                "Department loaded"
            );
            logs.push(PnCacheLog {
                department: dpt.to_string(),
                item_count: count,
                from_cache: true,
            });
            continue;
        }
        let data = PnResult::fetch_filter_dpt(state, dpt)
            .await
            .map_err(PnError::DatabaseError)?;

        let count = data.len();

        // 3. Save to Cache
        store
            .put::<PnResult>(state, &filename, data)
            .map_err(PnError::CacheError)?;

        info!(
            department = dpt,
            count = count,
            source = "database",
            "Department fetched and cached"
        );
        logs.push(PnCacheLog {
            department: dpt.to_string(),
            item_count: count,
            from_cache: false,
        });
    }
    Ok(logs)
}
