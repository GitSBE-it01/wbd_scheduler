use crate::core::drawing_catalog::error::SyncError;
use crate::core::drawing_catalog::model::Catalog;
use crate::system::cache;
use crate::system::db::{DcDrawing, QadItemMstr};
use crate::system::state::AppState;
use std::path::Path;
use tracing::info;

pub fn compile(
    scanned_items: Vec<Catalog>,
    items: Vec<QadItemMstr>,
) -> Result<Vec<DcDrawing>, SyncError> {
    let mut compile_scans: Vec<DcDrawing> = Vec::with_capacity(scanned_items.len());
    for item in items {
        if item.pt_status.as_deref() == Some("OBSOLETE") {
            continue;
        }
        let target_part = item.pt_part.trim().to_lowercase();
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

        compile_scans.push(DcDrawing {
            item: item.pt_part.clone(),
            item_desc: Some(item.desc.clone()),
            drawing_path: pth,
        });
    }
    info!(compile = ?compile_scans.len(), "finish compiling drawing catalog");
    Ok(compile_scans)
}
