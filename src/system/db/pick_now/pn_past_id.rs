use crate::system::{db::error::DbError, state::AppState};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PnPastID {
    pub wo_id: String,
    pub id_new: String,
}

use crate::system::db::DatabaseCacheable;
impl DatabaseCacheable for PnPastID {
    async fn fetch_db(state: &AppState) -> Result<Vec<Self>, DbError> {
        let pool = state.pool();
        let datas = sqlx::query_as::<_, Self>(
            r#"
                    SELECT 
                        DISTINCT (lot__id) as 'wo_id', 
                        id_new as 'id_new'
                        FROM dbpick_now.result_fix
                        WHERE data_date = ?
                        AND lot__id IS NOT NULL 
                        AND TRIM(lot__id) <> ''
                        GROUP BY lot__id
                "#,
        )
        .bind(state.clock.date_stringfy(-1, None))
        .fetch_all(pool)
        .await?;
        Ok(datas)
    }
}

use crate::system::cache::Cacheable;
use std::time::Duration;
impl Cacheable for PnPastID {
    type Key = String;

    fn namespace() -> &'static str {
        "pn" // Saved to ./cache_dir/scans/<key>.json
    }

    fn ttl() -> i64 {
        60 * 60 * 12 // Fresh for 1 hour
    }
}
