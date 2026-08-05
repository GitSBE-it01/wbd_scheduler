use crate::{app_error::AppError, app_state::AppState};
use sqlx::Row;
use tracing::info;

pub async fn run(state: &AppState) -> Result<(), AppError> {
    let pool = state.pool();
    let mut tx = pool.begin().await?;
    let data = sqlx::query("SELECT * FROM db_wbd.wbd_test WHERE id = 1")
        .fetch_one(&mut *tx)
        .await?;
    tx.commit().await?;
    let sum: i64 = data.get("id");
    info!("id= {}", sum);
    Ok(())
}
