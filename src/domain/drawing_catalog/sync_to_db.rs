use crate::{
    core::state::AppState,
    domain::drawing_catalog::{error::SyncError, model::ScannedItem},
};
use sqlx::{MySql, QueryBuilder};
use std::borrow::Cow;
use tracing::{error, info};

pub async fn insert_into_database(
    state: &AppState,
    records: Vec<ScannedItem>,
) -> Result<usize, SyncError> {
    if records.is_empty() {
        error!("!!>> No Data To Entry...");
        return Ok(0);
    }
    let pool = state.pool();

    let mut tx = pool.begin().await.map_err(SyncError::FailInsert)?;
    let mut total_rows_affected: u64 = 0;
    for chunk in records.chunks(1000) {
        let mut query_builder: QueryBuilder<MySql> = QueryBuilder::new(
            "INSERT INTO db_wbd.pn_drawing (item, item_desc, drawing_path, data_date) ",
        );

        query_builder.push_values(chunk, |mut b, rec| {
            // Safe conversion without temporary lifetime issues
            let path_str = rec
                .drawing_path
                .as_ref()
                .map_or(Cow::Borrowed("No Drawing"), |p| p.to_string_lossy());
            b.push_bind(&rec.item)
                .push_bind(&rec.item_desc)
                .push_bind(path_str.as_ref())
                .push_bind(state.clock.today());
        });

        let result = query_builder
            .build()
            .execute(&mut *tx)
            .await
            .map_err(SyncError::FailInsert)?;

        total_rows_affected += result.rows_affected();
    }

    // 2. Commit transaction after all chunks succeed
    tx.commit().await.map_err(SyncError::FailInsert)?;

    let total_inserted = usize::try_from(total_rows_affected).unwrap_or(usize::MAX);
    info!(insert_data = total_inserted, "-->> Insertion Done");

    Ok(total_inserted)
}

pub async fn check_if_today_data_exists(state: &AppState) -> Result<bool, SyncError> {
    let pool = state.pool();
    let has_data: bool = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1 
            FROM db_wbd.pn_drawing
            WHERE data_date = ?
        )
    "#,
    )
    .bind(state.clock.today())
    .fetch_one(pool)
    .await
    .map_err(SyncError::ScalarFetchFail)?;
    Ok(has_data)
}

pub async fn truncate_catalog_tables(state: &AppState) -> Result<(), SyncError> {
    let pool = state.pool();
    sqlx::query("TRUNCATE TABLE db_wbd.pn_drawing")
        .execute(pool)
        .await
        .map_err(SyncError::TruncateFailed)?;
    Ok(())
}
