use crate::pick_now::model::PIC;
use crate::{app_error::AppResult, app_state::AppState};

pub async fn run(state: &AppState) -> AppResult<Vec<PIC>> {
    let pool = state.pool();
    let datas: Vec<PIC> = sqlx::query_as::<_, PIC>(
        r#"
                SELECT 
                    DISTINCT (tipe) as 'tipe', 
                    optr as 'pic'
                    FROM dbpick_now.pic_part
                    ORDER BY tipe
            "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(datas)
}
