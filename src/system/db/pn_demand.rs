use crate::system::{db::error::DbError, state::AppState};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::path::Path;
use time::Date;
time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PnDemand {
    pub item: String,
    pub lot_id: String,
    pub loc_line: String,
    pub dept: String,
    pub qty: f64,
    #[serde(with = "date_format::option")]
    pub _date: Option<Date>,
    #[serde(with = "date_format::option")]
    pub rel_dt: Option<Date>,
    #[serde(with = "date_format::option")]
    pub due_dt: Option<Date>,
    pub wo_rmks: String,
    pub item_id: String,
    pub old_id: String,
}

impl PnDemand {
    pub async fn fetch_db(state: &AppState) -> Result<Vec<Self>, DbError> {
        let pool = state.pool();
        let datas = sqlx::query_as::<_, Self>(
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
}
