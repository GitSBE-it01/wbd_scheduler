use crate::system::{db::error::DbError, state::AppState};
use time;
use time::Date;
time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct QadPriceList {
    pub id: i32,
    pub pc_part: String,
    pub pc_list: String,
    pub pc_curr: Option<String>,
    pub pc_um: Option<String>,
    pub pc_desc: Option<String>,
    #[serde(with = "date_format::option")]
    pub pc_start: Option<Date>,
    #[serde(with = "date_format::option")]
    pub pc_expire: Option<Date>,
    pub pc_amt_type: Option<String>,
    pub pc_min_qty_01: Option<String>,
    pub pc_min_qty_02: Option<String>,
    pub pc_min_qty_03: Option<String>,
    pub pc_min_qty_04: Option<String>,
    pub pc_min_qty_05: Option<String>,
    pub pc_min_qty_06: Option<String>,
    pub pc_min_qty_07: Option<String>,
    pub pc_min_qty_08: Option<String>,
    pub pc_min_qty_09: Option<String>,
    pub pc_min_qty_10: Option<String>,
    pub pc_amt_01: Option<String>,
    pub pc_amt_02: Option<String>,
    pub pc_amt_03: Option<String>,
    pub pc_amt_04: Option<String>,
    pub pc_amt_05: Option<String>,
    pub pc_amt_06: Option<String>,
    pub pc_amt_07: Option<String>,
    pub pc_amt_08: Option<String>,
    pub pc_amt_09: Option<String>,
    pub pc_amt_10: Option<String>,
}

use crate::system::db::DatabaseCacheable;
impl DatabaseCacheable for QadPriceList {
    async fn fetch_db(state: &AppState) -> Result<Vec<Self>, DbError> {
        let pool = state.pool();
        let datas = sqlx::query_as::<_, Self>(
            r#"
            SELECT  
                id, 
                pc_list, 
                pc_curr, 
                pc_part, 
                pc_um, 
                pc_desc, 
                pc_startx as 'pc_start',
                pc_expirex as 'pc_expire',
                pc_amt_type, 
                `pc_min_qty[01]` AS 'pc_min_qty_01',
                `pc_min_qty[02]` AS 'pc_min_qty_02',
                `pc_min_qty[03]` AS 'pc_min_qty_03',
                `pc_min_qty[04]` AS 'pc_min_qty_04',
                `pc_min_qty[05]` AS 'pc_min_qty_05',
                `pc_min_qty[06]` AS 'pc_min_qty_06',
                `pc_min_qty[07]` AS 'pc_min_qty_07',
                `pc_min_qty[08]` AS 'pc_min_qty_08',
                `pc_min_qty[09]` AS 'pc_min_qty_09',
                `pc_min_qty[10]` AS 'pc_min_qty_10',
                `pc_amt[01]` AS 'pc_amt_01',
                `pc_amt[02]` AS 'pc_amt_02',
                `pc_amt[03]` AS 'pc_amt_03',
                `pc_amt[04]` AS 'pc_amt_04',
                `pc_amt[05]` AS 'pc_amt_05',
                `pc_amt[06]` AS 'pc_amt_06',
                `pc_amt[07]` AS 'pc_amt_07',
                `pc_amt[08]` AS 'pc_amt_08',
                `pc_amt[09]` AS 'pc_amt_09',
                `pc_amt[10]` AS 'pc_amt_10'
            FROM dbqad_live.pc_mstr
            "#,
        )
        .fetch_all(pool)
        .await?;
        Ok(datas)
    }
}

use crate::system::cache::Cacheable;
use std::time::Duration;
impl Cacheable for QadPriceList {
    type Key = String;

    fn namespace() -> &'static str {
        "qad" // Saved to ./cache_dir/scans/<key>.json
    }
    fn ttl() -> i64 {
        60 * 60 * 12 // Fresh for 1 hour
    }
}
