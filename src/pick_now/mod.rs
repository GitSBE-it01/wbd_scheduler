use crate::{app_error::AppResult, app_state::AppState};
use std::time::Instant;
use tracing::info;
pub mod cek_past_id;
pub mod demand;
pub mod inventory;
use model::Combine;
pub mod model;
pub mod pic;
mod process;

pub async fn run(state: &AppState) -> AppResult<()> {
    info!("==========Pick Now==========");
    let total_start = Instant::now();
    let has_data: bool = sqlx::query_as::<_, Combine>(
        r#"
        SELECT 
            item AS item,
            _desc AS desc1,
            remark AS remark,
            lot__id AS lot_id,
            loc__line AS loc_line,
            dept AS dept,
            qty AS qty,
            valAcc AS val_acc,
            qty_OH AS qty_oh,
            lotOH AS lot_oh,
            CAST(_date AS DATE) AS _date,
            CAST(rel_dt AS DATE) AS rel_dt,
            CAST(due_dt AS DATE) AS due_dt,
            wo_rmks AS wo_rmks,
            pic AS pic,
            pick AS pick,
            id_new AS id_new,
            item_id AS item_id,
            _desc2 AS desc2,
            old_id AS old_id
        FROM dbpick_now.result_fix WHERE data_date = CURDATE()
        LIMIT 1
        "#,
    )
    .fetch_optional(state.pool())
    .await?
    .is_some();
    if has_data == false {
        process::run(state).await?;
    };
    process::caching::run(state).await?;
    let total_dur = total_start.elapsed();
    info!(
        duration = ?total_dur,
        "====================END Pick Now===================="
    );
    Ok(())
}
