use crate::core::{error::AppResult, state::AppState};
use std::time::Instant;
use tracing::info;
pub mod cek_past_id;
pub mod demand;
pub mod inventory;
pub mod model;
pub mod pic;
mod process;

pub async fn run(state: &AppState) -> AppResult<()> {
    info!("==========Pick Now==========");
    let total_start = Instant::now();
    let has_data: bool = sqlx::query_scalar::<_, i32>(
        r#"
    SELECT 1 
    FROM dbpick_now.result_fix 
    WHERE data_date = ? 
    LIMIT 1 
    "#,
    )
    .bind(state.clock.today())
    .fetch_optional(state.pool())
    .await?
    .is_some();
    if has_data == false {
        process::run(state).await?;
    };
    process::caching::run(state).await?;
    let total_dur = total_start.elapsed();
    info!(
        duration = ?total_dur,
        "====================END Pick Now===================="
    );
    Ok(())
}
