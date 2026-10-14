use crate::system::{db::error::DbError, state::AppState};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time;
use time::Date;
time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct QadPsMstr {
    pub id: i32,
    pub ps_par: String,
    pub ps_comp: String,
    pub ps_qty_per: f64,
    #[serde(with = "date_format::option")]
    pub ps_start: Option<Date>,
    #[serde(with = "date_format::option")]
    pub ps_end: Option<Date>,
    pub ps_ps_code: Option<String>,
}

use crate::system::db::DatabaseCacheable;
impl DatabaseCacheable for QadPsMstr {
    async fn fetch_db(state: &AppState) -> Result<Vec<Self>, DbError> {
        let pool = state.pool();
        let datas = sqlx::query_as::<_, Self>(
            r#"
            SELECT  
            id,
            ps_par,
            ps_comp,
            ps_qty_per,
            ps_startx as 'ps_start',
            ps_endx as 'ps_end',
            ps_ps_code
            FROM dbqad_live.ps_mstr
            "#,
        )
        .fetch_all(pool)
        .await?;
        Ok(datas)
    }
}

use crate::system::cache::Cacheable;
use std::time::Duration;
impl Cacheable for QadPsMstr {
    type Key = String;

    fn namespace() -> &'static str {
        "qad" // Saved to ./cache_dir/scans/<key>.json
    }
    fn ttl() -> i64 {
        60 * 60 * 12 // Fresh for 1 hour
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::error::AppError;
    use crate::system::cache;
    use crate::system::cache::error::CacheError;
    use crate::system::config::AppConfig;
    use crate::system::db::qad::bom::QadPsMstr;
    #[tokio::test]
    async fn test_bom() -> Result<(), AppError> {
        let config = AppConfig::new(".env")?;
        let state = AppState::new(config).await?;
        let store = cache::CacheEngine::new("./cache");
        store
            .db_handler::<QadPsMstr>(&state, "bom".to_string())
            .await?;
        Ok(())
    }
}
