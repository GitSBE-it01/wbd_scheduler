use crate::system::{db::error::DbError, state::AppState};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySql, QueryBuilder};
use std::borrow::Cow;
use std::path::PathBuf;
use time;
use time::Date;
use tracing::error;
time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct JigMaster {
    pub jig_master_code: String,
    pub jig_drawing: Option<PathBuf>,
}

impl JigMaster {
    pub async fn update(state: &AppState, records: &[Self]) -> Result<u64, DbError> {
        if records.is_empty() {
            error!("No Record to insert");
            return Ok(0);
        }
        let pool = state.pool();
        let mut total_rows_affected = 0;
        for chunk in records.chunks(1000) {
            let mut query_builder: QueryBuilder<MySql> = QueryBuilder::new(
                "UPDATE db_wbd.jig_master SET jig_drawing = CASE
          ",
            );
            for rec in chunk {
                let path_str: Cow<'_, str> = rec
                    .jig_drawing
                    .as_deref()
                    .map_or(Cow::Borrowed("No Drawing"), |p| p.to_string_lossy());

                query_builder.push("WHEN jig_master_code = ");
                query_builder.push_bind(&rec.jig_master_code); // replace `id` / `rec.id` with your actual key
                query_builder.push(" THEN ");
                query_builder.push_bind(path_str.into_owned());
                query_builder.push(" ");
            }
            query_builder.push("ELSE jig_drawing END WHERE jig_master_code IN (");
            let mut separated = query_builder.separated(", ");
            for rec in chunk {
                separated.push_bind(&rec.jig_master_code);
            }
            separated.push_unseparated(")");
            let result = query_builder.build().execute(pool).await?;
            total_rows_affected += result.rows_affected();
        }
        Ok(total_rows_affected)
    }
}
