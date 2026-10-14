use crate::system::{db::error::DbError, state::AppState};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySql, QueryBuilder};
use time;
use time::Date;
time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");
use crate::system::cache::Cacheable;
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DBItemDevPrntRaw {
    pub item: String,
    pub parent: Option<String>,
    pub wo_lot: Option<String>,
    pub wo_nbr: Option<String>,
    pub rel_date: Option<Date>,
}

use crate::system::db::DatabaseCacheable;
impl DatabaseCacheable for DBItemDevPrntRaw {
    async fn fetch_db(state: &AppState) -> Result<Vec<Self>, DbError> {
        let pool = state.pool();
        let datas = sqlx::query_as::<_, Self>(
            r#"
                SELECT 
                    bs.item, 
                    bs.parent,
                    c.wo_lot,
                    c.wo_nbr,
                    c.min_date as 'rel_date'
                FROM (
                    SELECT 
                        a.item,
                        b.component_item,
                        b.parent_item,
                        CONCAT(b.parent_item, ' (', c.pt_desc1, ' ', c.pt_desc2, ')') AS 'parent' 
                    FROM db_wbd.item_dev_master a
                    JOIN qad_ebom.ebom b
                        ON a.item = b.component_item 
                    AND b.end_effective = '-'
                    LEFT JOIN dbqad_live.pt_mstr c
                    ON b.parent_item = c.pt_part
                    WHERE a.status = 'open' -- Added table alias 'a.' for strict safety
                    ORDER BY b.component_item, b.parent_item ASC
                ) bs
                LEFT JOIN (
                    SELECT w1.wo_part, w1.wo_nbr, w1.wo_lot, w2.min_date
                    FROM dbqad_live.wo_mstr w1
                    JOIN (
                        SELECT wo_part, MIN(wo_rel_datex) AS min_date
                        FROM dbqad_live.wo_mstr
                        WHERE wo_status != 'C'
                        GROUP BY wo_part
                    ) w2 
                    ON w1.wo_part = w2.wo_part AND w1.wo_rel_datex = w2.min_date
                    WHERE w1.wo_status != 'C'
                ) c 
                ON bs.parent_item = c.wo_part
                ORDER BY bs.item ASC, bs.parent ASC
            "#,
        )
        .fetch_all(pool)
        .await?;
        Ok(datas)
    }
}

impl Cacheable for DBItemDevPrntRaw {
    type Key = String;

    fn namespace() -> &'static str {
        "item_dev"
    }

    fn ttl() -> i64 {
        60 * 60 * 15 // Fresh for 1 hour
    }
}
