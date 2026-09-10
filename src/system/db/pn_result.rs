use crate::system::{db::error::DbError, state::AppState};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySql, QueryBuilder};
use time;
use time::Date;
time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, PartialEq)]
pub struct PnResult {
    pub item: String,
    pub _desc: Option<String>,
    pub remark: String,
    pub lot__id: Option<String>,
    pub loc__line: Option<String>,
    pub dept: String,
    pub qty: Decimal,
    pub valAcc: Decimal,
    pub qty_OH: Decimal,
    pub lotOH: Option<String>,
    #[serde(with = "date_format::option")]
    pub _date: Option<Date>,
    #[serde(with = "date_format::option")]
    pub rel_dt: Option<Date>,
    #[serde(with = "date_format::option")]
    pub due_dt: Option<Date>,
    pub wo_rmks: Option<String>,
    pub pic: Option<String>,
    pub pick: String,
    pub id_new: String,
    pub item_id: Option<String>,
    pub _desc2: Option<String>,
    pub old_id: Option<String>,
}

impl PnResult {
    pub async fn fetch_db_with_dept(state: &AppState, dpt: &str) -> Result<Vec<Self>, DbError> {
        let pool = state.pool();
        let records = sqlx::query_as::<_, Self>(
            r#"
            SELECT 
            a.item,
            a._desc,
            a.remark,
            a.lot__id,
            a.loc__line,
            a.dept,
            a.qty,
            a.valAcc,
            a.qty_OH,
            a.lotOH,
            CAST(a._date AS DATE) AS '_date',
            CAST(a.rel_dt AS DATE) AS 'rel_dt',
            CAST(a.due_dt AS DATE) AS 'due_dt',
            a.wo_rmks,
            a.pic,
            a.pick,
            a.id_new,
            a.item_id,
            a._desc2,
            a.old_id
        FROM dbpick_now.result_fix a
        INNER JOIN (
            SELECT DISTINCT (item)
            FROM dbpick_now.result_fix 
            WHERE dept = ? AND data_date = ?
        ) b ON a.item = b.item
        WHERE a.data_date = ?
          AND a.dept IN (?, 'WH')
        ORDER BY 
            a.item ASC, 
            (a.dept = 'WH') DESC, 
            remark ASC, 
            _date ASC, 
            due_dt ASC, 
            lot__id ASC
            "#,
        )
        .bind(dpt)
        .bind(state.clock.today())
        .bind(state.clock.today())
        .bind(dpt)
        .fetch_all(pool)
        .await?;
        Ok(records)
    }

    pub async fn check_curr_data(state: &AppState) -> Result<bool, DbError> {
        let pool = state.pool();
        let has_data: bool = sqlx::query_scalar::<_, i32>(
            r#"
            SELECT 1 
            FROM dbpick_now.result_fix 
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
            return Err(DbError::NoRecordInsert);
        }
        let pool = state.pool();
        let mut total_rows_affected = 0;
        for chunk in records.chunks(1000) {
            let mut query_builder: QueryBuilder<MySql> = QueryBuilder::new(
            "INSERT INTO dbpick_now.result_fix(
                data_date, item, _desc, remark, lot__id, loc__line, dept, qty, valAcc, qty_OH, lotOH, _date, rel_dt, due_dt, wo_rmks, pic, pick, id_new, item_id, _desc2, old_id
            ) 
          "
        );
            query_builder.push_values(chunk, |mut b, rec| {
                b.push_bind(state.clock.today())
                    .push_bind(&rec.item)
                    .push_bind(&rec._desc)
                    .push_bind(&rec.remark)
                    .push_bind(&rec.lot__id)
                    .push_bind(&rec.loc__line)
                    .push_bind(&rec.dept)
                    .push_bind(rec.qty)
                    .push_bind(rec.valAcc)
                    .push_bind(rec.qty_OH)
                    .push_bind(&rec.lotOH)
                    .push_bind(rec._date)
                    .push_bind(rec.rel_dt)
                    .push_bind(rec.due_dt)
                    .push_bind(&rec.wo_rmks)
                    .push_bind(&rec.pic)
                    .push_bind(&rec.pick)
                    .push_bind(&rec.id_new)
                    .push_bind(&rec.item_id)
                    .push_bind(&rec._desc2)
                    .push_bind(&rec.old_id);
            });

            let result = query_builder.build().execute(pool).await?;
            total_rows_affected += result.rows_affected();
        }
        Ok(total_rows_affected)
    }

    pub async fn delete(state: &AppState) -> Result<u64, DbError> {
        let pool = state.pool();
        let del_past = sqlx::query("DELETE FROM dbpick_now.result_fix WHERE data_date = ?")
            .bind(state.clock.date_stringfy(-3, None))
            .execute(pool)
            .await?;
        let rows_deleted = del_past.rows_affected();
        Ok(rows_deleted)
    }

    pub async fn fetch_filter_dpt(state: &AppState, dpt: &str) -> Result<Vec<Self>, DbError> {
        let pool = state.pool();
        let sql = r#"
        SELECT 
            a.item,
            a._desc,
            a.remark,
            a.lot__id,
            a.loc__line,
            a.dept,
            a.qty,
            a.valAcc,
            a.qty_OH,
            a.lotOH,
            CAST(a._date AS DATE) AS '_date',
            CAST(a.rel_dt AS DATE) AS 'rel_dt',
            CAST(a.due_dt AS DATE) AS 'due_dt',
            a.wo_rmks,
            a.pic,
            a.pick,
            a.id_new,
            a.item_id,
            a._desc2,
            a.old_id
        FROM dbpick_now.result_fix a
        INNER JOIN (
            SELECT DISTINCT (item)
            FROM dbpick_now.result_fix 
            WHERE dept = ? AND data_date = ?
        ) b ON a.item = b.item
        WHERE a.data_date = ?
          AND a.dept IN (?, 'WH')
        ORDER BY 
            a.item ASC, 
            (a.dept = 'WH') DESC, 
            remark ASC, 
            _date ASC, 
            due_dt ASC, 
            lot__id ASC
    "#;
        let db_datas: Vec<Self> = sqlx::query_as::<_, Self>(sql)
            .bind(dpt)
            .bind(state.clock.today())
            .bind(state.clock.today())
            .bind(dpt)
            .fetch_all(pool)
            .await?;
        Ok(db_datas)
    }
}

use crate::system::cache::Cacheable;
impl Cacheable for PnResult {
    type Key = String;

    fn namespace() -> &'static str {
        "pick_now_2" // Saved to ./cache_dir/scans/<key>.json
    }

    fn ttl() -> i64 {
        60 * 60 * 18 // Fresh for 1 hour
    }
}
