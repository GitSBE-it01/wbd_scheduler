use crate::pick_now::model::Inv;
use crate::{app_error::AppResult, app_state::AppState};

pub async fn run(state: &AppState) -> AppResult<Vec<Inv>> {
    let pool = state.pool();
    let datas: Vec<Inv> = sqlx::query_as::<_, Inv>(
            r#"
                SELECT 
                    inv_a.ld_part AS 'item', 
                    CASE 
                        WHEN inv_b.loc_department = 'PROD' THEN 'P1.ASSY'
                        WHEN inv_b.loc_loc = 'WHPR' THEN 'WH_ASSY'
                        ELSE inv_b.loc_department
                        END
                    AS 'dept',
                    CAST('2000-01-01' AS DATE) as '_date',
                    SUM(inv_a.ld_qty_oh) AS 'qty_oh',
                    GROUP_CONCAT(
                    CONCAT(
                        inv_a.ld_lot, '-', inv_a.ld_loc, '=', 
                        IF(inv_a.ld_qty_oh LIKE '.%', CONCAT('0', inv_a.ld_qty_oh), CAST(inv_a.ld_qty_oh AS CHAR))
                    ) 
                    SEPARATOR '--')
                    AS 'lot_oh'
                FROM dbqad_live.ld_det inv_a 
                JOIN dbqad_live.loc_mstr inv_b  
                    ON inv_a.ld_loc = inv_b.loc_loc
                WHERE inv_a.ld_status NOT IN ('AVALAN', 'WKNDA', 'UNCOUNT', 'TRANSIT', 'CONSIGN', 'SCRAP', 'REJECT', 'SAMPLE')  
                GROUP BY inv_a.ld_part, inv_b.loc_department
                ORDER BY inv_a.ld_part ASC, inv_b.loc_department
            "#,
        )
        .fetch_all(pool)
        .await?;

    Ok(datas)
}

/*
use crate::pick_now::model::Inv;
use crate::{
    app_error::AppResult,
    app_state::AppState,
    app_utilities::{save_file::save_as_json, validation::is_cache_valid},
};
use std::path::Path;
use std::time::Instant;
use tokio::fs;
use tracing::info;

pub async fn run(state: &AppState) -> AppResult<Vec<Inv>> {
    let pool = state.pool();
    let query_start = Instant::now();
    let file_path = Path::new("./cache/");
    let filename = "inventory.json";
    let fullpath = file_path.join(filename);
    let datas: Vec<Inv> = if is_cache_valid(&fullpath, None).await? {
        info!("Cache hit! Reading data from {:?}", fullpath);
        let json_content = fs::read_to_string(&fullpath).await?;
        serde_json::from_str(&json_content)?
    } else {
        info!("Cache missed or expired. Querying database...");
        let db_datas: Vec<Inv> = sqlx::query_as::<_, Inv>(
            r#"
                SELECT 
                    inv_a.ld_part AS 'item', 
                    CASE 
                        WHEN inv_b.loc_department = 'PROD' THEN 'P1.ASSY'
                        WHEN inv_b.loc_loc = 'WHPR' THEN 'WH_ASSY'
                        ELSE inv_b.loc_department
                        END
                    AS 'dept',
                    CAST('2000-01-01' AS DATE) as '_date',
                    SUM(inv_a.ld_qty_oh) AS 'qty_oh',
                    GROUP_CONCAT(
                    CONCAT(
                        inv_a.ld_lot, '-', inv_a.ld_loc, '=', 
                        IF(inv_a.ld_qty_oh LIKE '.%', CONCAT('0', inv_a.ld_qty_oh), CAST(inv_a.ld_qty_oh AS CHAR))
                    ) 
                    SEPARATOR '--')
                    AS 'lot_oh'
                FROM dbqad_live.ld_det inv_a 
                JOIN dbqad_live.loc_mstr inv_b  
                    ON inv_a.ld_loc = inv_b.loc_loc
                WHERE inv_a.ld_status NOT IN ('AVALAN', 'WKNDA', 'UNCOUNT', 'TRANSIT', 'CONSIGN', 'SCRAP', 'REJECT', 'SAMPLE')  
                GROUP BY inv_a.ld_part, inv_b.loc_department
                ORDER BY inv_a.ld_part ASC, inv_b.loc_department
            "#,
        )
        .fetch_all(pool)
        .await?;
        save_as_json(&db_datas, file_path, filename).await?;

        db_datas
    };
    let query_duration = query_start.elapsed();
    info!(
        "Fetch Inventory data running duration: {:?}",
        query_duration
    );
    println!("Loaded {} records. Ready for processing...", datas.len());
    Ok(datas)
}
*/
