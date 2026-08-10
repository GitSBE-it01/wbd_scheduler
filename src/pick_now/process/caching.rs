use crate::pick_now::model::Combine;
use crate::{
    app_error::AppResult,
    app_state::AppState,
    app_utilities::{save_file::save_as_json, validation::is_cache_valid},
};
use std::path::Path;
use tracing::info;

pub async fn run(state: &AppState) -> AppResult<()> {
    let pool = state.pool();
    info!("Caching data databases...");
    let depts = [
        "P1.ASSY", "PROD1.VC", "WH_ASSY", "SBE3", "SBE4", "PROD2", "PROD3", "QA", "SUBCON", "SBE4",
        "WOODWORK",
    ];
    let sql = r#"
        SELECT 
            a.item AS item,
            a._desc AS desc1,
            a.remark AS remark,
            a.lot__id AS lot_id,
            a.loc__line AS loc_line,
            a.dept AS dept,
            a.qty AS qty,
            a.valAcc AS val_acc,
            a.qty_OH AS qty_oh,
            a.lotOH AS lot_oh,
            CAST(a._date AS DATE) AS _date,
            CAST(a.rel_dt AS DATE) AS rel_dt,
            CAST(a.due_dt AS DATE) AS due_dt,
            a.wo_rmks AS wo_rmks,
            a.pic AS pic,
            a.pick AS pick,
            a.id_new AS id_new,
            a.item_id AS item_id,
            a._desc2 AS desc2,
            a.old_id AS old_id
        FROM dbpick_now.result_fix a
        INNER JOIN (
            SELECT DISTINCT (item)
            FROM dbpick_now.result_fix 
            WHERE dept = ? AND data_date = CURDATE()
        ) b ON a.item = b.item
        WHERE a.data_date = CURDATE()
          AND a.dept IN (?, 'WH')
        ORDER BY 
            a.item ASC, 
            (a.dept = 'WH') DESC, 
            remark ASC, 
            _date ASC, 
            due_dt ASC, 
            lot__id ASC
    "#;
    let file_path = Path::new("./cache/");
    for dpt in depts {
        info!("Starting Caching data {dpt} databases...");
        let filename = format!("pn_cache_reslt__{dpt}.json");
        let fullpath = file_path.join(&filename);
        if is_cache_valid(&fullpath, Some(18)).await? == false {
            let db_datas: Vec<Combine> = sqlx::query_as::<_, Combine>(sql)
                .bind(dpt)
                .bind(dpt)
                .fetch_all(pool)
                .await?;
            save_as_json(&db_datas, file_path, &filename).await?;
        }
    }
    Ok(())
}
