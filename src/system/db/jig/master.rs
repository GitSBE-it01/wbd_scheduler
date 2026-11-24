use crate::system::{db::error::DbError, state::AppState};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySql, QueryBuilder};
use time;
use time::Date;
use tracing::error;
time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DbJigMaster {
    pub item_jig: String,
    pub desc_jig: Option<String>,
    pub status_jig: Option<String>,
    pub material: Option<String>,
    pub jig_type: String,
    pub drawing: Option<String>,
}

use crate::system::db::DatabaseCacheable;
impl DatabaseCacheable for DbJigMaster {
    async fn fetch_db(state: &AppState) -> Result<Vec<Self>, DbError> {
        let pool = state.pool();
        let datas = sqlx::query_as::<_, Self>(
            r#"
            SELECT  
                item_jig,
                desc_jig,
                status_jig,
                material,
                type as 'jig_type',
                drawing
            FROM db_jig.jig_master
            "#,
        )
        .fetch_all(pool)
        .await?;
        Ok(datas)
    }
}

use crate::system::cache::Cacheable;
use std::time::Duration;
impl Cacheable for DbJigMaster {
    type Key = String;

    fn namespace() -> &'static str {
        "db_jig" // Saved to ./cache_dir/scans/<key>.json
    }
    fn ttl() -> i64 {
        60 * 60 * 12 // Fresh for 1 hour
    }
}
