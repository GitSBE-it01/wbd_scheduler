use crate::system::{db::error::DbError, state::AppState};
time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ItemDevDetail {
    pub item_number: String,
    pub item_desc: String,
    pub buyer: String,
    pub item_status: String,
    pub prod_line: String,
    pub pm_code: String,
    pub need_isir: Option<String>,
    pub status_isir: Option<String>,
    pub bom_cek: bool,
    pub routing_cek: bool,
    pub cost_cek: bool,
}

use crate::system::db::DatabaseCacheable;
impl DatabaseCacheable for ItemDevDetail {
    async fn fetch_db(state: &AppState) -> Result<Vec<Self>, DbError> {
        let pool = state.pool();
        let datas = sqlx::query_as::<_, Self>(
            r#"
            SELECT  
            pt.pt_part AS 'item_number',
            CONCAT(pt.pt_desc1, ' ', pt.pt_desc2) as 'item_desc',
            pt.pt_status AS 'item_status',
            pt.pt_buyer AS 'buyer',
            pt.pt_prod_line AS 'prod_line',
            pt.pt__log01 AS 'need_isir',
            pt.pt__chr01 AS 'status_isir',
            pt.pt_pm_code AS 'pm_code',
            EXISTS (
                SELECT 1 
                FROM dbqad_live.ro_det ro 
                WHERE ro.ro_routing = pt.pt_part
            ) AS 'rout_cek',

            EXISTS (
                SELECT 1 
                FROM dbqad_live.ps_mstr ps 
                WHERE ps.ps_par = pt.pt_part
            ) AS 'bom_cek',
            EXISTS (
                SELECT 1 
                FROM dbqad_live.pc_mstr pc 
                WHERE pc.pc_part = pt.pt_part
            ) AS 'rout_cek',
            ps_par AS 'bom_cek',
            pc_part AS 'cost_cek'
            FROM dbqad_live.pt_mstr pt
            "#,
        )
        .fetch_all(pool)
        .await?;
        Ok(datas)
    }
}

use crate::system::cache::Cacheable;
use std::time::Duration;
impl Cacheable for ItemDevDetail {
    type Key = String;

    fn namespace() -> &'static str {
        "item_dev" // Saved to ./cache_dir/scans/<key>.json
    }
    fn ttl() -> i64 {
        60 * 60 * 12 // Fresh for 1 hour
    }
}
