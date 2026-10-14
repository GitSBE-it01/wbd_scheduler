use crate::system::{db::error::DbError, state::AppState};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySql, QueryBuilder};
use time;
use time::Date;
time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");
use crate::system::cache::Cacheable;
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ItemDevParentData {
    pub itd_master_id: i32,
    pub parent_ls: Option<String>,
    pub closest_wo_nbr: Option<String>,
    pub closest_wo_lot: Option<String>,
    pub closest_rel_date: Option<Date>,
}

impl ItemDevParentData {
    pub async fn inserts(state: &AppState, records: Vec<Self>) -> Result<u64, DbError> {
        if records.is_empty() {
            return Err(DbError::NoRecordInsert);
        }
        let pool = state.pool();
        let mut total_rows_affected = 0;
        for chunk in records.chunks(1000) {
            let mut query_builder: QueryBuilder<MySql> = QueryBuilder::new(
                "INSERT INTO db_wbd.item_dev_parent_data(itd_master_id, parent_ls, closest_wo_nbr, closest_wo_lot, closest_rel_date) ",
            );
            query_builder.push_values(chunk, |mut b, rec| {
                b.push_bind(rec.itd_master_id)
                    .push_bind(&rec.parent_ls)
                    .push_bind(&rec.closest_wo_nbr)
                    .push_bind(&rec.closest_wo_lot)
                    .push_bind(rec.closest_rel_date);
            });
            query_builder.push(" ON DUPLICATE KEY UPDATE ");
            query_builder.push("parent_ls = VALUES(parent_ls), ");
            query_builder.push("closest_wo_nbr = VALUES(closest_wo_nbr),");
            query_builder.push("closest_wo_lot = VALUES(closest_wo_lot), ");
            query_builder.push("closest_rel_date = VALUES(closest_rel_date)");

            let result = query_builder.build().execute(pool).await?;
            total_rows_affected += result.rows_affected();
        }
        Ok(total_rows_affected)
    }
}
