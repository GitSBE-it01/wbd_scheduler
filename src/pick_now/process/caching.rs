use crate::pick_now::model::CachePickNow;
use crate::{
    app_error::AppResult,
    app_state::AppState,
    app_utilities::{save_file::save_as_json, validation::is_cache_valid},
};
use std::path::Path;

pub async fn run(state: &AppState) -> AppResult<()> {
    let pool = state.pool();
    let depts = [
        "P1.ASSY", "PROD1.VC", "WH_ASSY", "SBE3", "SBE4", "PROD2", "PROD3", "QA", "SUBCON", "SBE4",
        "WOODWORK",
    ];
    let sql = r#"
        SELECT 
            a.item,
            a._desc,
            a.remark,
            a.lot__id,
            a.loc__line,
            a.dept,
            a.qty,
            a.valAcc,
            a.qty_OH,
            a.lotOH,
            CAST(a._date AS DATE) AS '_date',
            CAST(a.rel_dt AS DATE) AS 'rel_dt',
            CAST(a.due_dt AS DATE) AS 'due_dt',
            a.wo_rmks,
            a.pic,
            a.pick,
            a.id_new,
            a.item_id,
            a._desc2,
            a.old_id
        FROM dbpick_now.result_fix a
        INNER JOIN (
            SELECT DISTINCT (item)
            FROM dbpick_now.result_fix 
            WHERE dept = ? AND data_date = ?
        ) b ON a.item = b.item
        WHERE a.data_date = ?
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
        let filename = format!("pn_cache_reslt__{dpt}.json");
        let fullpath = file_path.join(&filename);
        if is_cache_valid(&fullpath, Some(18)).await? == false {
            let db_datas: Vec<CachePickNow> = sqlx::query_as::<_, CachePickNow>(sql)
                .bind(dpt)
                .bind(state.clock.today())
                .bind(state.clock.today())
                .bind(dpt)
                .fetch_all(pool)
                .await?;
            save_as_json(&db_datas, file_path, &filename).await?;
        }
    }
    Ok(())
}
