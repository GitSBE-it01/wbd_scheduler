use crate::system::cache::Cacheable;
use crate::system::{db::error::DbError, state::AppState};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::path::Path;
use std::time::Duration;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PnPic {
    pub tipe: String,
    pub pic: String,
}
impl PnPic {
    pub async fn fetch_db(state: &AppState) -> Result<Vec<Self>, DbError> {
        let pool = state.pool();
        let datas = sqlx::query_as::<_, Self>(
            r#"
                SELECT 
                    DISTINCT (tipe) as 'tipe', 
                    optr as 'pic'
                    FROM dbpick_now.pic_part
                    ORDER BY tipe
            "#,
        )
        .fetch_all(pool)
        .await?;
        Ok(datas)
    }
}

impl Cacheable for PnPic {
    type Key = String;

    fn namespace() -> &'static str {
        ""
    }

    fn ttl() -> Duration {
        Duration::from_secs(60 * 60 * 12) // 1 hour
    }
}


