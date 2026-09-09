use crate::system::cache::Cacheable;
use crate::system::{db::error::DbError, state::AppState};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::path::Path;
use std::time::Duration;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct EBomParent {
    pub comp: String,
    pub comp_desc: String,
    pub comp_stat: String,
    pub parent_item: String,
    pub parent_desc: String,
    pub parent_stat: String,
}

use crate::system::db::DatabaseCacheable;

impl DatabaseCacheable for EBomParent {
    async fn fetch_db(state: &AppState) -> Result<Vec<Self>, DbError> {
        let pool = state.pool();
        let datas = sqlx::query_as::<_, Self>(
            r#"
                SELECT 
                    a.component_item as 'comp',
                    a.component_desc as 'comp_desc',
                    a.Stats as 'comp_stat',
                    a.parent_item as 'parent_item',
                    a.parent_desc as 'parent_desc',
                    b.pt_status as 'parent_stat'
                FROM qad_ebom.ebom a
                JOIN dbqad_live.pt_mstr b 
                    ON a.parent_item = b.pt_part 
                WHERE a.component_item NOT LIKE '1%' 
                ORDER BY a.component_item ASC
            "#,
        )
        .fetch_all(pool)
        .await?;
        Ok(datas)
    }
}

impl Cacheable for EBomParent {
    type Key = String;

    fn namespace() -> &'static str {
        "ebom_parent" // Saved to ./cache_dir/scans/<key>.json
    }

    fn ttl() -> i64 {
        60 * 60 * 18 // Fresh for 1 hour
    }
}
