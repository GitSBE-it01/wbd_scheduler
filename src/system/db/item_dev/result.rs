use crate::system::{db::error::DbError, state::AppState};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySql, QueryBuilder};
use time;
use time::Date;
time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemDevStatus {
    Open,
    Close,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CostStatus {
    Ok,
    SBE4,
    Purchasing,
    NoCost,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ItemDevMaster {
    pub item: String,
    pub bom: bool,
    pub routing: bool,
    pub cost: CostStatus,
    pub status: ItemDevStatus,
    pub no_srf: i32,
    pub no_sample: Option<i32>,
}

impl ItemDevMaster {
    pub async fn upsert(state: &AppState, records: &[Self]) -> Result<u64, DbError> {
        if records.is_empty() {
            return Err(DbError::NoRecordInsert);
        }
        let pool = state.pool();
        let mut total_rows_affected = 0;
        for chunk in records.chunks(1000) {
            let mut query_builder: QueryBuilder<MySql> = QueryBuilder::new(
                "INSERT INTO db_wbd.item_dev_master(item, bom, routing, cost,  status, no_srf, no_sample, added, mdf_date) ",
            );
            query_builder.push_values(chunk, |mut b, rec| {
                let cost: &str = match &rec.cost {
                    CostStatus::Ok => "Ok",
                    CostStatus::SBE4 => "Belum ada harga dari SBE4",
                    CostStatus::Purchasing => "Belum ada harga dari PROCUREMENT",
                    _ => "-",
                };
                let status = match &rec.status {
                    ItemDevStatus::Open => "Open",
                    _ => "Close",
                };
                b.push_bind(&rec.item)
                    .push_bind(rec.bom)
                    .push_bind(rec.routing)
                    .push_bind(cost)
                    .push_bind(status)
                    .push_bind(rec.no_srf)
                    .push_bind(Some(rec.no_sample))
                    .push_bind(state.clock.today())
                    .push_bind(state.clock.today());
            });
            query_builder.push(" ON DUPLICATE KEY UPDATE ");
            query_builder.push("bom = VALUES(bom), ");
            query_builder.push("routing = VALUES(routing),");
            query_builder.push("cost = VALUES(cost), ");
            query_builder.push("status = VALUES(status), ");
            query_builder.push("no_srf = VALUES(no_srf), ");
            query_builder.push("no_sample = VALUES(no_sample), ");
            query_builder.push("close_date = IF(VALUES(status) = 'Close', CURDATE(), NULL), ");
            query_builder.push("mdf_date = CURDATE()");

            let result = query_builder.build().execute(pool).await?;
            total_rows_affected += result.rows_affected();
        }
        Ok(total_rows_affected)
    }
}

use crate::system::cache::Cacheable;
impl Cacheable for ItemDevMaster {
    type Key = String;

    fn namespace() -> &'static str {
        "item_dev"
    }

    fn ttl() -> i64 {
        60 * 60 * 15 // Fresh for 1 hour
    }
}
