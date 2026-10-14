use crate::system::{db::error::DbError, state::AppState};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time;
use time::Date;
time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct QadWoMstr {
    pub id: i32,
    pub wo_nbr: Option<String>,
    pub wo_lot: Option<String>,
    pub wo_status: Option<String>,
    pub wo_part: Option<String>,
    pub wo_qty_ord: Option<String>,
    pub wo__chr04: Option<String>,
    pub wo_so_job: Option<String>,
    pub wo_rmks: Option<String>,
    pub wo_vend: Option<String>,
    pub wo_qty_comp: Option<String>,
    pub wo_acct_close: Option<String>,
    pub wo_rel_date: Option<String>,
    pub wo_due_date: Option<String>,
    pub wo_stat_close_date: Option<String>,
    pub wo_close_date: Option<String>,
    pub wo_ord_date: Option<String>,
    pub wo_routing: Option<String>,
    pub wo_qty_rjct: Option<String>,
}

use crate::system::db::DatabaseCacheable;
impl DatabaseCacheable for QadWoMstr {
    async fn fetch_db(state: &AppState) -> Result<Vec<Self>, DbError> {
        let pool = state.pool();
        let datas = sqlx::query_as::<_, Self>(
            r#"
            SELECT 
            id, 
            wo_nbr AS 'work_order', 
            wo_lot as 'wo_id', 
            wo_status, 
            wo_part, 
            wo_qty_ord, 
            wo__chr04 as 'assy_line', 
            wo_so_job, 
            wo_rmks, 
            wo_vend, 
            wo_qty_comp, 
            wo_acct_close, 
            wo_rel_datex AS 'wo_rel_date', 
            wo_due_datex AS 'wo_due_date', 
            wo_stat_close_datex AS 'wo_stat_close_date', 
            wo_close_datex AS 'wo_close_date', 
            wo_ord_datex AS 'wo_ord_date', 
            wo_routing, 
            wo_qty_rjct 
            FROM dbqad_live.wo_mstr 
            "#,
        )
        .fetch_all(pool)
        .await?;
        Ok(datas)
    }
}

use crate::system::cache::Cacheable;
use std::time::Duration;
impl Cacheable for QadWoMstr {
    type Key = String;

    fn namespace() -> &'static str {
        "qad" // Saved to ./cache_dir/scans/<key>.json
    }
    fn ttl() -> i64 {
        60 * 60 * 12 // Fresh for 1 hour
    }
}
