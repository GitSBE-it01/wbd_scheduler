use crate::system::{db::error::DbError, state::AppState};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySql, QueryBuilder};
use std::borrow::Cow;
use std::path::PathBuf;
use tracing::error;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DcDrawing {
    pub item: String,
    pub item_desc: Option<String>,
    pub drawing_path: Option<PathBuf>,
}
impl DcDrawing {
    pub async fn check_curr_data(state: &AppState) -> Result<bool, DbError> {
        let pool = state.pool();
        let has_data: bool = sqlx::query_scalar::<_, i32>(
            r#"
            SELECT 1 
            FROM db_wbd.pn_drawing 
            WHERE data_date = ? 
            LIMIT 1 
            "#,
        )
        .bind(state.clock.today())
        .fetch_optional(pool)
        .await?
        .is_some();
        Ok(has_data)
    }

    pub async fn insert(state: &AppState, records: &[Self]) -> Result<u64, DbError> {
        if records.is_empty() {
            error!("No Record to insert");
            return Ok(0);
        }
        let pool = state.pool();
        let mut total_rows_affected = 0;
        for chunk in records.chunks(1000) {
            let mut query_builder: QueryBuilder<MySql> = QueryBuilder::new(
                "INSERT INTO db_wbd.pn_drawing(item, item_desc, drawing_path, data_date) 
          ",
            );
            query_builder.push_values(chunk, |mut b, rec| {
                let path_str = rec
                    .drawing_path
                    .as_ref()
                    .map_or(Cow::Borrowed("No Drawing"), |p| p.to_string_lossy());
                b.push_bind(&rec.item)
                    .push_bind(&rec.item_desc)
                    .push_bind(path_str.as_ref())
                    .push_bind(state.clock.today());
            });

            let result = query_builder.build().execute(pool).await?;
            total_rows_affected += result.rows_affected();
        }
        Ok(total_rows_affected)
    }

    pub async fn delete(state: &AppState) -> Result<u64, DbError> {
        let pool = state.pool();
        let delete = sqlx::query("TRUNCATE TABLE db_wbd.pn_drawing")
            .execute(pool)
            .await?;
        let rows_deleted = delete.rows_affected();
        Ok(rows_deleted)
    }
}

use crate::system::db::DatabaseCacheable;
#[derive(Debug, Clone, FromRow)]
struct DcDrawingQuery {
    pub item: String,
    pub item_desc: Option<String>,
    pub drawing_path: Option<String>,
}

impl DatabaseCacheable for DcDrawing {
    async fn fetch_db(state: &AppState) -> Result<Vec<Self>, DbError> {
        let pool = state.pool();
        let datas = sqlx::query_as::<_, DcDrawingQuery>(
            r#"
            SELECT 
                item,
                item_desc,
                drawing_path
            FROM db_wbd.pn_drawing
            WHERE data_date = ?
            "#,
        )
        .bind(state.clock.today())
        .fetch_all(pool)
        .await?;
        let result = datas
            .into_iter()
            .map(|row| Self {
                item: row.item,
                item_desc: row.item_desc,
                drawing_path: row.drawing_path.map(PathBuf::from),
            })
            .collect();
        Ok(result)
    }
}

use crate::system::cache::Cacheable;
use std::time::Duration;
impl Cacheable for DcDrawing {
    type Key = String;

    fn namespace() -> &'static str {
        "pick_now"
    }

    fn ttl() -> i64 {
        5
    }
}
