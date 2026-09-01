use crate::system::{cache::FileCacheable, db::error::DbError, state::AppState};
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

impl FileCacheable for PnPic {
    fn cache_filename(key: Option<&str>) -> String {
        if let Some(key) = key {
            format!("pn_pic__{}.json", key)
        } else {
            format!("pn_pic.json")
        }
    }

    fn default_ttl() -> Duration {
        Duration::from_secs(60 * 60 * 12) // 1 day
    }

    fn cache_sub_dir() -> &'static Path {
        &Path::new("./cache/")
    }
}
