use crate::core::{error::AppResult, state::AppState};
use crate::domain::pick_now::model::Demand;

pub async fn run(state: &AppState) -> AppResult<Vec<Demand>> {
    let pool = state.pool();
    let datas: Vec<Demand> = sqlx::query_as::<_, Demand>(
        r#"
            SELECT 
                dm_a.wod_part AS item, 
                dm_a.wod_lot AS lot_id,
                dm_b.wo__chr04 AS loc_line,
                dm_c.dept AS dept,
                (dm_a.wod_qty_req - dm_a.wod_qty_iss) AS qty,
                dm_b.wo_rel_datex AS _date,
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
    Ok(datas)
}
