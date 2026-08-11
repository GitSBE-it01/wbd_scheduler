use crate::{
    app_error::AppResult,
    app_state::AppState,
    app_utilities::{save_file::save_as_json, validation::is_cache_valid},
    qad_gen::item_data,
};
use std::path::Path;
use std::time::Instant;
use tracing::{error, info};

mod model;
use model::ItemData;
mod inserts;
mod recursive;
use recursive::get_filenames_from_dir;

pub async fn run(state: &AppState) -> AppResult<()> {
    info!("====================Drawing====================");
    let total_start = Instant::now();

    let target_directory = r"\\192.168.2.219\General\Drawing\Release For Production\";
    let file_path = Path::new("./cache/");
    let filename = "cache_data__drawing.json";
    let fullpath = file_path.join(filename);
    let has_data: bool = sqlx::query_as::<_, ItemData>(
        r#"
        SELECT  
            item,
            item_desc,
            drawing_path
        FROM db_wbd.pn_drawing WHERE data_date = CURDATE()
        LIMIT 1
        "#,
    )
    .fetch_optional(state.pool())
    .await?
    .is_some();
    if has_data == false {
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
            let mut item_data_list: Vec<ItemData> = Vec::with_capacity(files.len());
            let pt_mstr = item_data::run(state).await?;
            for file in &files {
                // Get filename without allocating a Vec
                let item_name = file.rsplit('\\').next().unwrap_or(file).to_string();

                // Safely build full network path
                let drawing_path = Path::new(target_directory)
                    .join(file)
                    .to_string_lossy()
                    .into_owned();
                let desc = pt_mstr
                    .iter()
                    .find(|dtx| dtx.pt_part == item_name)
                    .map(|dtx| dtx.desc.clone());
                item_data_list.push(ItemData {
                    item: item_name,
                    item_desc: desc,
                    drawing_path,
                });
            }

            // Save processed data to cache
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
