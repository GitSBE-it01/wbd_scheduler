use crate::system::{db::error::DbError, state::AppState};
time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ItemDevException {
    pub id: i32,
    pub itd_master_id: i32,
    pub item: String,
    pub exception: bool,
}

use crate::system::db::DatabaseCacheable;
impl DatabaseCacheable for ItemDevException {
    async fn fetch_db(state: &AppState) -> Result<Vec<Self>, DbError> {
        let pool = state.pool();
        let datas = sqlx::query_as::<_, Self>(
            r#"
            SELECT 
                a.id,
                a.itd_master_id,
                b.item,
                a.exception
            FROM db_wbd.item_dev_exception a
            JOIN db_wbd.item_dev_master b
            ON a.itd_master_id = b.id
            ORDER BY itd_master_id
            "#,
        )
        .fetch_all(pool)
        .await?;
        Ok(datas)
    }
}

use crate::system::cache::Cacheable;
use std::time::Duration;
impl Cacheable for ItemDevException {
    type Key = String;

    fn namespace() -> &'static str {
        "item_dev" // Saved to ./cache_dir/scans/<key>.json
    }
    fn ttl() -> i64 {
        60 * 60 * 12 // Fresh for 1 hour
    }
}
