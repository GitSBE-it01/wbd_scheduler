use crate::pick_now::model::CekID;
use crate::{
    app_error::AppResult,
    app_state::AppState,
    app_utilities::{save_file::save_as_json, validation::is_cache_valid},
};
use std::path::Path;
use std::time::Instant;
use tokio::fs;
use tracing::info;

pub async fn run(state: &AppState) -> AppResult<Vec<CekID>> {
    let pool = state.pool();
    let query_start = Instant::now();
    let file_path = Path::new("./cache/");
    let filename = "past_rslt.json";
    let fullpath = file_path.join(filename);
    let datas: Vec<CekID> = if is_cache_valid(&fullpath, None).await? {
        info!("Cache hit! Reading data from {:?}", fullpath);
        let json_content = fs::read_to_string(&fullpath).await?;
        serde_json::from_str(&json_content)?
    } else {
        info!("Cache missed or expired. Querying database...");
        let db_datas: Vec<CekID> = sqlx::query_as::<_, CekID>(
            r#"
                SELECT 
                    DISTINCT (lot__id) as 'wo_id', 
                    id_new as 'id_new'
                    FROM dbpick_now.result_fix
                    WHERE data_date = CURDATE( ) - INTERVAL 1 DAY 
                    AND lot__id IS NOT NULL 
                    AND TRIM(lot__id) <> ''
                    GROUP BY lot__id
            "#,
        )
        .fetch_all(pool)
        .await?;
        save_as_json(&db_datas, file_path, filename).await?;

        db_datas
    };
    let query_duration = query_start.elapsed();
    info!(
        "Fetch past result data running duration: {:?}",
        query_duration
    );
    println!("Loaded {} records. Ready for processing...", datas.len());
    Ok(datas)
}
