use crate::system::{db::error::DbError, state::AppState};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PnPastID {
    pub wo_id: String,
    pub id_new: String,
}

impl PnPastID {
    pub async fn fetch_db(state: &AppState) -> Result<Vec<Self>, DbError> {
        let pool = state.pool();
        let datas = sqlx::query_as::<_, Self>(
            r#"
                    SELECT 
                        DISTINCT (lot__id) as 'wo_id', 
                        id_new as 'id_new'
                        FROM dbpick_now.result_fix
                        WHERE data_date = ?
                        AND lot__id IS NOT NULL 
                        AND TRIM(lot__id) <> ''
                        GROUP BY lot__id
                "#,
        )
        .bind(state.clock.today())
        .fetch_all(pool)
        .await?;
        Ok(datas)
    }
}
