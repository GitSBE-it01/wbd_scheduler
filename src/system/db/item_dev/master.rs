use crate::system::{db::error::DbError, state::AppState};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySql, QueryBuilder};
use time;
use time::Date;
time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");
use crate::system::cache::Cacheable;
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DBItemDevMstr {
    pub id: i32,
    pub item: String,
    pub bom: bool,
    pub routing: bool,
    pub cost: String,
    pub status: String,
    pub no_srf: i32,
    pub no_sample: i32,
}

use crate::system::db::DatabaseCacheable;
impl DatabaseCacheable for DBItemDevMstr {
    async fn fetch_db(state: &AppState) -> Result<Vec<Self>, DbError> {
        let pool = state.pool();
        let datas = sqlx::query_as::<_, Self>(
            r#"
                SELECT 
                    id,
                    item, 
                    bom, 
                    routing, 
                    cost, 
                    status, 
                    no_srf, 
                    no_sample,
                    added,
                    close_date,
                    mdf_date
                FROM db_wbd.item_dev_master
                WHERE status = 'open'
                ORDER BY item
            "#,
        )
        .fetch_all(pool)
        .await?;
        Ok(datas)
    }
}

impl Cacheable for DBItemDevMstr {
    type Key = String;

    fn namespace() -> &'static str {
        "item_dev"
    }

    fn ttl() -> i64 {
        60 * 60 * 15 // Fresh for 1 hour
    }
}
