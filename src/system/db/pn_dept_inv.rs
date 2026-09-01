use crate::system::{db::error::DbError, state::AppState};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::Date;
time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PnInv {
    pub item: String,
    pub dept: String,
    pub qty_oh: f64,
    #[serde(with = "date_format::option")]
    pub _date: Option<Date>,
    pub lot_oh: String,
}
impl PnInv {
    pub async fn fetch_db(state: &AppState) -> Result<Vec<Self>, DbError> {
        let pool = state.pool();
        let datas = sqlx::query_as::<_, Self>(
            r#"
                SELECT 
                    inv_a.ld_part AS 'item', 
                    CASE 
                        WHEN inv_b.loc_department = 'PROD' THEN 'P1.ASSY'
                        WHEN inv_b.loc_loc = 'WHPR' THEN 'WH_ASSY'
                        ELSE inv_b.loc_department
                        END
                    AS 'dept',
                    CAST('2000-01-01' AS DATE) as '_date',
                    SUM(inv_a.ld_qty_oh) AS 'qty_oh',
                    GROUP_CONCAT(
                    CONCAT(
                        inv_a.ld_lot, '-', inv_a.ld_loc, '=', 
                        IF(inv_a.ld_qty_oh LIKE '.%', CONCAT('0', inv_a.ld_qty_oh), CAST(inv_a.ld_qty_oh AS CHAR))
                    ) 
                    SEPARATOR '--')
                    AS 'lot_oh'
                FROM dbqad_live.ld_det inv_a 
                JOIN dbqad_live.loc_mstr inv_b  
                    ON inv_a.ld_loc = inv_b.loc_loc
                WHERE inv_a.ld_status NOT IN ('AVALAN', 'WKNDA', 'UNCOUNT', 'TRANSIT', 'CONSIGN', 'SCRAP', 'REJECT', 'SAMPLE')  
                GROUP BY inv_a.ld_part, inv_b.loc_loc
                ORDER BY inv_a.ld_part ASC, inv_b.loc_department
            "#,
            )
            .fetch_all(pool)
            .await?;

        Ok(datas)
    }
}
