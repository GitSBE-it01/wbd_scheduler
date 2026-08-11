use std::path::Path;
use std::time::Instant;
use tokio::fs;
use tracing::{error, info};
use wbd_scheduler::{
    app_config::AppConfig,
    app_error::{AppError, AppResult},
    app_state::AppState,
    app_utilities::{save_file::save_as_json, validation::is_cache_valid},
    pick_now::model::Combine,
};

#[tokio::test]
async fn ck_pick_now() -> AppResult<()> {
    let config = AppConfig::new(".env")?;
    let state = AppState::new(config).await?;
    let pool = state.pool();
    let query_start = Instant::now();
    let file_path = Path::new("./cache/");
    let mut filename = "curr_data.json";
    let fullpath = file_path.join(filename);
    let mut pass_result: Vec<Combine> = if is_cache_valid(&fullpath, None).await? {
        info!("Cache hit! Reading data from {:?}", fullpath);
        let json_content = fs::read_to_string(&fullpath).await?;
        serde_json::from_str(&json_content)?
    } else {
        info!("Cache missed or expired. Querying database...");
        let db_datas: Vec<Combine> = sqlx::query_as::<_, Combine>(
            r#"
            SELECT item, 
            _desc as 'desc1',
            remark, 
            lot__id as 'lot_id', 
            loc__line as 'loc_line', 
            dept, 
            qty, 
            valAcc as 'val_acc', 
            qty_OH as 'qty_oh', 
            lotOH as 'lot_oh', 
            CAST(_date AS DATE) as '_date', 
            CAST(rel_dt AS DATE) as 'rel_dt', 
            CAST(due_dt AS DATE) as 'due_dt', 
            wo_rmks, 
            pic, 
            pick, 
            id_new, 
            item_id, 
            _desc2 AS desc2,
            old_id 
            FROM dbpick_now.result_fix WHERE data_date=CURDATE()
            "#,
        )
        .fetch_all(pool)
        .await?;
        save_as_json(&db_datas, file_path, filename).await?;

        db_datas
    };
    let query_duration = query_start.elapsed();
    info!("Get past result: {:?}", query_duration);
    filename = "rs_result.json";
    let fullpath = file_path.join(filename);
    let mut new_result: Vec<Combine> = if is_cache_valid(&fullpath, None).await? {
        info!("Cache hit! Reading data from {:?}", fullpath);
        let json_content = fs::read_to_string(&fullpath).await?;
        serde_json::from_str(&json_content)?
    } else {
        error!("Cache miss at {:?}", fullpath);
        return Err(AppError::CacheMiss(fullpath));
    };
    pass_result.sort_by(|a, b| {
        a.dept
            .cmp(&b.dept)
            .then_with(|| a.item.cmp(&b.item))
            .then_with(|| a._date.cmp(&b._date))
    });
    new_result.sort_by(|a, b| {
        a.dept
            .cmp(&b.dept)
            .then_with(|| a.item.cmp(&b.item))
            .then_with(|| a._date.cmp(&b._date))
    });
    assert_eq!(
        pass_result.len(),
        new_result.len(),
        "Record count mismatch! past: {}, New: {}",
        pass_result.len(),
        new_result.len()
    );
    assert_eq!(
        pass_result, new_result,
        "Data discrepancy found between legacy DB data and new process output"
    );
    Ok(())
}
