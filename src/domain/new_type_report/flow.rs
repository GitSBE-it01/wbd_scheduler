use crate::{
    core::state::AppState,
    core::utilities::{read_cache::read_data, save_file::save_as_json, validation::is_cache_valid},
    domain::{
        new_type_report::{
            error::NtrError,
            model::{NtrResult, NtrSummary},
            sync_to_db::insert_into_database,
        },
        qad_data,
    },
};
use std::path::Path;
use tracing::info;

pub async fn run_catalog(
    state: &AppState,
    cmd: SyncDirectoryCommand,
) -> Result<NtrResult, NtrError> {
    let has_todays_data = check_if_today_data_exists(state).await?;
    if has_todays_data {
        return Ok(SyncOutcome::Skipped {
            reason: "Catalog already contains data for today".to_string(),
        });
    }
    truncate_catalog_tables(state).await?;
    info!("->> Scanning Dir");
    let scanned_items = parse_directory(&cmd.target_dir).await?;
    let items_found = scanned_items.len();
    info!(count= ?items_found, "Scanning Result");
    let items = qad_data::pt_mstr::ItemMstr::get(state)
        .await
        .map_err(SyncError::Database)?;
    let file_path = Path::new("./cache/");
    let filename = "cache_data__drawing.json";
    let fullpath = file_path.join(filename);
    let compile_scans: Vec<ScannedItem> = if is_cache_valid(&fullpath, None).await? {
        let db_datas: Vec<ScannedItem> = read_data(&fullpath).await?;
        db_datas
    } else {
        let mut db_datas: Vec<ScannedItem> = Vec::with_capacity(scanned_items.len());
        info!("->> Compiling Scan");
        for item in items {
            // 1. Normalize target part: trim whitespace and convert to lowercase
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

            db_datas.push(ScannedItem {
                item: item.pt_part.clone(),
                item_desc: Some(item.desc.clone()),
                drawing_path: pth,
            });
        }
        save_as_json(&db_datas, file_path, filename).await?;
        db_datas
    };

    info!("->> Start Inserting");
    let items_inserted = insert_into_database(state, compile_scans).await?;
    info!(count_inserted= ?items_inserted, "Inserting Result");
    // Step 3: Return the final Output
    Ok(SyncOutcome::Completed(SyncSummary {
        items_found: items_found, // Simplified for example
        items_inserted: items_inserted,
    }))
}
