use crate::{
    core::state::AppState,
    core::utilities::{read_cache::read_data, save_file::save_as_json, validation::is_cache_valid},
    domain::{
        drawing_catalog::{
            error::SyncError,
            model::{ScannedItem, SyncDirectoryCommand, SyncOutcome, SyncSummary},
            scan_dir::parse_directory,
            sync_to_db::{
                check_if_today_data_exists, insert_into_database, truncate_catalog_tables,
            },
        },
        qad_data,
    },
};
use std::path::Path;
use tracing::info;

pub async fn run_catalog(
    state: &AppState,
    cmd: SyncDirectoryCommand,
) -> Result<SyncOutcome, SyncError> {
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
    let file_path = Path::new("//192.168.2.103/xampp/htdocs/wbd/_cache/pick_now/");
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

/*
pub async fn run(state: &AppState) -> AppResult<()> {
    info!("====================Drawing====================");
    let total_start = Instant::now();

    let target_directory = r"\\192.168.2.219\General\Drawing\Release For Production\";
    let file_path = Path::new("//192.168.2.103/xampp/htdocs/wbd/_cache/pick_now/");
    let filename = "cache_data__drawing.json";
    let fullpath = file_path.join(filename);
    let has_data: bool = sqlx::query_scalar::<_, i32>(
        r#"
    SELECT 1 
    FROM db_wbd.pn_drawing 
    WHERE data_date = ? 
    LIMIT 1
    "#,
    )
    .bind(state.clock.today())
    .fetch_optional(state.pool())
    .await?
    .is_some();
    println!("check has_data = {:#?}", has_data);
    if has_data == false {
        info!("Running drawing processed");
        // Return early on error to prevent overwriting existing cache file
        let files = match get_filenames_from_dir(target_directory) {
            Ok(files) => files,
            Err(err) => {
                error!(
                    error = ?err,
                    directory = %target_directory,
                    "!!>> Failed to read directory; preserving existing cache"
                );
                return Err(err); // Or return Ok(()) if you want non-fatal exit
            }
        };

        if files.is_empty() {
            error!(directory = %target_directory, "!!>> No files found or directory is empty");
        } else {
            let target_path = Path::new(target_directory);
            let mut item_data_list: Vec<ItemData> = Vec::with_capacity(files.len());
            let pt_mstr = item_data::run(state).await?;
            for part in pt_mstr {
                if matches!(part.pt_status.as_deref(), Some("OBSOLETE")) {
                    continue;
                }
                let item_lower = part.pt_part.to_lowercase();
                let drawing_path = files
                    .iter()
                    .find(|file| file.to_lowercase().contains(&item_lower))
                    .map(|file| target_path.join(file).to_string_lossy().into_owned())
                    .unwrap_or_else(|| "No Drawing".to_string());
                item_data_list.push(ItemData {
                    item: part.pt_part,
                    item_desc: Some(part.desc),
                    drawing_path,
                });
            }
            if !is_cache_valid(&fullpath, Some(18)).await? {
                save_as_json(&item_data_list, file_path, filename).await?;
            }
            inserts::run(state, item_data_list).await?;
        }
    }

    let total_dur = total_start.elapsed();
    info!(
        duration = ?total_dur,
        "====================END Drawing===================="
    );

    Ok(())
}

*/
