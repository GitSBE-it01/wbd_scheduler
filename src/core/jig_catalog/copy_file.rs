use crate::core::drawing_catalog::error::SyncError;
use crate::core::drawing_catalog::model::Catalog;
use crate::system::cache;
use crate::system::db::{
    DcDrawing,
    jig::{master::DbJigMaster, master_to_ins::JigMaster},
};
use crate::system::state::AppState;
use std::path::Path;
use tracing::info;

pub fn compile(
    scanned_items: Vec<Catalog>,
    items: Vec<DbJigMaster>,
) -> Result<Vec<JigMaster>, SyncError> {
    let mut compile_scans: Vec<JigMaster> = Vec::with_capacity(scanned_items.len());
    for item in items {
        let target_part = item.item_jig.trim().to_lowercase();
        let pth = scanned_items
            .iter()
            .find(|r| {
                let file_str = r
                    .file_path
                    .file_stem() // Uses filename without extension (e.g., "drawing" instead of "drawing.pdf")
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_lowercase();

                file_str.contains(&target_part)
            })
            .map(|found| found.file_path.clone());

        compile_scans.push(JigMaster {
            jig_master_code: item.item_jig.clone(),
            jig_drawing: pth,
        });
    }
    info!(compile = ?compile_scans.len(), "finish compiling drawing catalog");
    Ok(compile_scans)
}
