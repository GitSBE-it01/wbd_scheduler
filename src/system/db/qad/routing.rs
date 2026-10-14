use crate::system::{db::error::DbError, state::AppState};
use rust_decimal::Decimal;
use time;
use time::Date;
time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct QadRouting {
    pub id: i32,
    pub ro_routing: String,
    pub ro_op: String,
    pub ro_start: Option<String>,
    pub ro_end: Option<String>,
    pub ro_desc: Option<String>,
    pub ro_wkctr: Option<String>,
    pub ro_mch_op: Option<String>,
    pub ro_wait: Option<String>,
    pub ro_setup_men: Option<String>,
    pub ro_men_mch: Option<String>,
    pub ro_setup: Option<String>,
    pub ro_vend: Option<String>,
    pub work_center_2: Option<String>,
    pub ro_run: Option<String>,
    pub ro_sub_cost: Option<String>,
    pub ro_sub_lead: Option<String>,
    pub ro_mod_date: Option<String>,
    pub ro_mod_userid: Option<String>,
}

use crate::system::db::DatabaseCacheable;
impl DatabaseCacheable for QadRouting {
    async fn fetch_db(state: &AppState) -> Result<Vec<Self>, DbError> {
        let pool = state.pool();
        let datas = sqlx::query_as::<_, Self>(
            r#"
           SELECT  
            id,
            ro_routing,
            ro_op,
            ro_start,
            ro_end,
            ro_desc,
            ro_wkctr,
            ro_mch_op,
            ro_wait,
            ro_setup_men,
            ro_men_mch,
            ro_setup,
            ro_vend,
            ro__chr01 as 'work_center_2',
            ro_run,
            ro_sub_cost,
            ro_sub_lead,
            ro_mod_date,
            ro_mod_userid
            FROM dbqad_live.ro_det
            "#,
        )
        .fetch_all(pool)
        .await?;
        Ok(datas)
    }
}

use crate::system::cache::Cacheable;
use std::time::Duration;
impl Cacheable for QadRouting {
    type Key = String;

    fn namespace() -> &'static str {
        "qad" // Saved to ./cache_dir/scans/<key>.json
    }
    fn ttl() -> i64 {
        60 * 60 * 12 // Fresh for 1 hour
    }
}
