use crate::core::{error::AppResult, state::AppState};
use crate::domain::pick_now::model::Combine;
use sqlx::{MySql, QueryBuilder};
use tracing::{error, info};

pub async fn run(state: &AppState, records: Vec<Combine>) -> AppResult<()> {
    if records.is_empty() {
        error!("!!>> No Data To Entry...");
        return Ok(());
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
                .push_bind(&rec.desc1)
                .push_bind(&rec.remark)
                .push_bind(&rec.lot_id)
                .push_bind(&rec.loc_line)
                .push_bind(&rec.dept)
                .push_bind(rec.qty)
                .push_bind(rec.val_acc)
                .push_bind(rec.qty_oh)
                .push_bind(&rec.lot_oh)
                .push_bind(rec._date)
                .push_bind(rec.rel_dt)
                .push_bind(rec.due_dt)
                .push_bind(&rec.wo_rmks)
                .push_bind(&rec.pic)
                .push_bind(&rec.pick)
                .push_bind(&rec.id_new)
                .push_bind(&rec.item_id)
                .push_bind(&rec.desc2)
                .push_bind(&rec.old_id);
        });

        let result = query_builder.build().execute(pool).await?;
        total_rows_affected += result.rows_affected();
    }
    info!(insert_data = total_rows_affected, "-->> Insertion Done");
    Ok(())
}
