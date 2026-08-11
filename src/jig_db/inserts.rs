use crate::drawing::model::ItemData;
use crate::{app_error::AppResult, app_state::AppState};
use sqlx::{MySql, QueryBuilder};
use tracing::{error, info};

pub async fn run(state: &AppState, records: Vec<ItemData>) -> AppResult<()> {
    if records.is_empty() {
        error!("!!>> No Data To Entry...");
        return Ok(());
    }
    let pool = state.pool();
    let mut total_rows_affected = 0;
    for chunk in records.chunks(1000) {
        let mut query_builder: QueryBuilder<MySql> = QueryBuilder::new(
            "INSERT INTO db_wbd.pn_drawing(
                item, item_desc, drawing_path, data_date
            ) 
          ",
        );
        query_builder.push_values(chunk, |mut b, rec| {
            b.push_bind(&rec.item)
                .push_bind(&rec.item_desc)
                .push_bind(&rec.drawing_path)
                .push_bind("CURDATE()");
        });

        let result = query_builder.build().execute(pool).await?;
        total_rows_affected += result.rows_affected();
    }
    info!(insert_data = total_rows_affected, "-->> Insertion Done");
    Ok(())
}
