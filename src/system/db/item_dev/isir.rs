use crate::system::{db::error::DbError, state::AppState};
time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ItemDevIsir {
    pub id_srf: i32,
    pub idmain: Option<String>,
    pub id_sample: Option<i32>,
    pub partrelease: Option<String>,
    pub partreleasedate: Option<String>,
    pub samplerelease: Option<String>,
    pub sample: Option<i32>,
    pub cost: Option<String>,
}

use crate::system::db::DatabaseCacheable;
impl DatabaseCacheable for ItemDevIsir {
    async fn fetch_db(state: &AppState) -> Result<Vec<Self>, DbError> {
        let pool = state.pool();
        let datas = sqlx::query_as::<_, Self>(
            r#"
            SELECT 
                a.id as id_srf,
                b.idmain,
                b.id as id_sample,
                TRIM(a.partrelease) as partrelease,
                a.partreleasedate,
                a.samplerelease,
                b.sample,
                b.cost
            FROM dbspec_1.isir_main a
            LEFT JOIN dbspec_1.isir_sample b
                on a.id = b.idmain
            WHERE a.partrelease !='' AND a.partrelease IS NOT NULL
            ORDER BY a.partrelease, STR_TO_DATE(a.partreleasedate, '%d/%m/%Y') DESC
            "#,
        )
        .fetch_all(pool)
        .await?;
        Ok(datas)
    }
}

use crate::system::cache::Cacheable;
use std::time::Duration;
impl Cacheable for ItemDevIsir {
    type Key = String;

    fn namespace() -> &'static str {
        "item_dev" // Saved to ./cache_dir/scans/<key>.json
    }
    fn ttl() -> i64 {
        60 * 60 * 12 // Fresh for 1 hour
    }
}
