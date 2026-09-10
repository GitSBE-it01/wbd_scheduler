pub mod error;
pub mod model;
pub mod process;
pub mod scan_dir;
use crate::system::db::dc_drawing::DcDrawing;
use crate::system::db::qad_item::QadItemMstr;
use crate::system::state::AppState;
use crate::system::{cache, db::DatabaseCacheable};
use error::SyncError;
use model::{SyncDirectoryCommand, SyncOutcome, SyncSummary};
use process::compile;
use scan_dir::parse_directory;
use std::path::Path;
use tracing::info;

pub async fn run_catalog(
    state: &AppState,
    cmd: SyncDirectoryCommand,
) -> Result<SyncOutcome, SyncError> {
    let store = cache::CacheEngine::new("./cache");
    let wbd_store = cache::CacheEngine::new("//192.168.2.103/xampp/htdocs/wbd/_cache/");
    let filename = "cache_data__drawing".to_string();
    let has_todays_data = DcDrawing::check_curr_data(state).await?;
    if has_todays_data {
        let data = wbd_store.db_handler::<DcDrawing>(state, filename).await?;
        info!(count = ?data.data.len(), "Drawing catalog stored");
        return Ok(SyncOutcome::Skipped {
            reason: "Catalog already contains data for today".to_string(),
        });
    }
    let del = DcDrawing::delete(state).await?;
    info!(count= ?del, "Deleting pn_drawing data");
    let scanned_items = parse_directory(&cmd.target_dir).await?;
    let itm_count = scanned_items.len();
    info!(scan_item= ?itm_count, "finish scan drawing dir");
    let items = store
        .db_handler::<QadItemMstr>(state, "items".to_string())
        .await?;

    let result: Vec<DcDrawing> =
        if let Some(cached_data) = wbd_store.get::<DcDrawing>(state, &filename) {
            cached_data.data
        } else {
            compile(scanned_items, items.data)?
        };
    info!(rslt_count=?result.len(), "starting inserting result data");
    let itm_inserted = if !result.is_empty() {
        let cnt = DcDrawing::insert(state, &result).await?;
        wbd_store.db_handler::<DcDrawing>(state, filename).await?;
        cnt.try_into().unwrap()
    } else {
        0
    };
    // Step 3: Return the final Output
    Ok(SyncOutcome::Completed(SyncSummary {
        items_found: itm_count, // Simplified for example
        items_inserted: itm_inserted,
    }))
}
