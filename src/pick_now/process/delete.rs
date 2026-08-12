use crate::{app_error::AppResult, app_state::AppState};
use tracing::info;

pub async fn run(state: &AppState) -> AppResult<()> {
    let pool = state.pool();

    let del_past = sqlx::query("DELETE FROM dbpick_now.result_fix WHERE data_date = ?")
        .bind(state.clock.date_stringfy(-3))
        .execute(pool)
        .await?;
    let rows_deleted = del_past.rows_affected();
    info!(delete_data = rows_deleted, "-->> Deletion Done");
    Ok(())
}
