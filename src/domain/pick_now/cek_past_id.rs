use crate::core::{error::AppResult, state::AppState};
use crate::domain::pick_now::model::CekID;

pub async fn run(state: &AppState) -> AppResult<Vec<CekID>> {
    let pool = state.pool();
    let datas: Vec<CekID> = sqlx::query_as::<_, CekID>(
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
