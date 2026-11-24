pub mod model;
pub mod process;
pub mod scan_dir_vec;
use crate::core::drawing_catalog::error::SyncError;
use crate::core::drawing_catalog::model::{SyncOutcome, SyncSummary};
use crate::system::db::{
    DcDrawing,
    jig::{master::DbJigMaster, master_to_ins::JigMaster},
};
use crate::system::state::AppState;
use crate::system::{cache, db::DatabaseCacheable};
use model::ScanDirectoryCommand;
pub use process::compile;
use scan_dir_vec::parse_directory_collection;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::info;

pub async fn jig_drawing_handle(
    state: &AppState,
    cmd: ScanDirectoryCommand,
) -> Result<SyncOutcome, SyncError> {
    let store = cache::CacheEngine::new("./cache");
    let wbd_store = cache::CacheEngine::new("//192.168.2.103/xampp/htdocs/wbd/_cache/");
    let filename = "cache_data__jig_drawing".to_string();
    info!("starting parse dir...");
    let scanned_items = parse_directory_collection(&cmd.target_dir).await?;
    let itm_count = scanned_items.len();
    info!(scan_item= ?itm_count, "finish scan drawing dir");
    let items: cache::CacheResult<DbJigMaster> = store
        .db_handler::<DbJigMaster>(state, "jig_master".to_string())
        .await?;

    info!("starting compile data...");
    let collection: Vec<JigMaster> = compile(scanned_items, items.data)?;
    let new_dir = Path::new("//192.168.2.103/xampp/htdocs/wbd/jig_db/images/");
    let mut result: Vec<JigMaster> = Vec::new();
    info!("copying file to wbd...");
    for rec in collection {
        let filename = format!("{}.pdf", rec.jig_master_code);
        let mut new_path: Option<PathBuf> = None;
        let dst_path = new_dir.join(&filename);
        // let src_path = rec.jig_drawing;
        let bytes_copied = if let Some(src_path) = &rec.jig_drawing {
            new_path = Some(Path::new("/wbd/jig_db/images/").join(&filename));
            fs::copy(src_path, &dst_path)?
        } else {
            0
        };

        result.push(JigMaster {
            jig_master_code: rec.jig_master_code,
            jig_drawing: new_path,
        })
    }
    info!(rslt_count=?result.len(), "starting inserting result data");
    let itm_inserted = if !result.is_empty() {
        let cnt = JigMaster::update(state, &result).await?;
        cnt.try_into().unwrap()
    } else {
        0
    };
    // Step 3: Return the final Output
    Ok(SyncOutcome::Completed(SyncSummary {
        items_found: itm_count, // Simplified for example
        items_inserted: 0,
    }))
}
