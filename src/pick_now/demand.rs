use crate::pick_now::model::Demand;
use crate::{
    app_error::AppResult,
    app_state::AppState,
    app_utilities::{save_file::save_as_json, validation::is_cache_valid},
};
use std::path::Path;
use std::time::Instant;
use time::Duration;
use tokio::fs;
use tracing::info;

pub async fn run(state: &AppState) -> AppResult<Vec<Demand>> {
    let pool = state.pool();
    let ttl = Duration::hours(24);
    let query_start = Instant::now();
    let file_path = Path::new("./cache/");
    let filename = "demand.json";
    let fullpath = file_path.join(filename);
    let datas: Vec<Demand> = if is_cache_valid(&fullpath, ttl).await? {
        info!("Cache hit! Reading data from {:?}", fullpath);
        let json_content = fs::read_to_string(&fullpath).await?;
        serde_json::from_str(&json_content)?
    } else {
        info!("Cache missed or expired. Querying database...");
        let db_datas: Vec<Demand> = sqlx::query_as::<_, Demand>(
            r#"
            SELECT 
                dm_a.wod_part AS item, 
                dm_a.wod_lot AS lot_id,
                dm_b.wo__chr04 AS loc_line,
                dm_c.dept AS dept,
                (dm_a.wod_qty_req - dm_a.wod_qty_iss) AS qty,
                dm_b.wo_rel_datex AS rel_dt,
                dm_b.wo_due_datex AS due_dt,
                dm_b.wo_rmks AS wo_rmks,
                dm_b.wo_part AS item_id,
                dm_b.wo_vend AS old_id
            FROM dbqad_live.wod_det dm_a 
            JOIN dbqad_live.wo_mstr dm_b 
                ON dm_a.wod_lot = dm_b.wo_lot
            JOIN dbpick_now.dept_new dm_c
                ON dm_b.wo__chr04 = dm_c.assyLine
            WHERE dm_b.wo_status = 'R' AND dm_b.wo__chr04 != '' 
            ORDER BY dm_b.wo_vend, dm_a.wod_lot, dm_a.wod_part
            "#,
        )
        .fetch_all(pool)
        .await?;
        save_as_json(&db_datas, file_path, filename).await?;

        db_datas
    };
    let query_duration = query_start.elapsed();
    info!("Fetch Demand data running duration: {:?}", query_duration);
    println!("Loaded {} records. Ready for processing...", datas.len());
    Ok(datas)
}
