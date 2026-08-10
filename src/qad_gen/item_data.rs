use crate::{
    app_error::AppResult,
    app_state::AppState,
    app_utilities::{save_file::save_as_json, validation::is_cache_valid},
};
//  use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::path::Path;
use std::time::Instant;
use tokio::fs;
use tracing::info;
time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PtMstr {
    pub id: i32,
    pub pt_part: String,
    pub desc: String,
    pub pt_dsgn_grp: Option<String>,
    pub pt_um: Option<String>,
    pub pt_buyer: Option<String>,
    pub pt_status: Option<String>,
    pub pt_size: Option<String>,
    pub pt_vend: Option<String>,
    pub pt_pm_code: Option<String>,
    pub pt_drwg_size: Option<String>,
    pub floorstock: Option<String>,
    pub pt_sfty_stk: Option<String>,
    pub pt_pur_lead: Option<String>,
    pub pt_mfg_lead: Option<String>,
    pub pt_insp_lead: Option<String>,
    pub pt_prod_line: Option<String>,
    pub pt_part_type: Option<String>,
    pub pt_group: Option<String>,
    pub pt_promo: Option<String>,
    pub pt_cum_lead: Option<String>,
    pub pt_draw: Option<String>,
    pub pt_drwg_loc: Option<String>,
    pub need_isir: Option<String>,
    pub status_isir: Option<String>,
    pub spsd_control: Option<String>,
    pub pt_ord_min: Option<String>,
    pub pt_iss_pol: Option<String>,
    pub pt_loc: Option<String>,
    pub pt_taxable: Option<String>,
    pub pt_taxc: Option<String>,
    pub pt_shelflife: Option<String>,
    pub pt_yield_pct: Option<String>,
    pub exp_lkl_oem: Option<String>,
    pub prod_category: Option<String>,
    pub construction: Option<String>,
    pub finishing: Option<String>,
    pub pt_added: Option<String>,
    pub project: Option<String>,
    pub pt_rev: Option<String>,
    pub pt_ms: Option<String>,
    pub pt_abc: Option<String>,
    pub pt_lot_ser: Option<String>,
    pub pt_rctpo_status: Option<String>,
    pub pt_rctpo_active: Option<String>,
    pub pt_rctwo_status: Option<String>,
    pub pt_rctwo_active: Option<String>,
    pub pt_sfty_time: Option<String>,
    pub pt_article: Option<String>,
    pub pt_ord_qty: Option<String>,
    pub pt_site: Option<String>,
}

pub async fn run(state: &AppState) -> AppResult<Vec<PtMstr>> {
    let pool = state.pool();
    let query_start = Instant::now();
    let file_path = Path::new("./cache/");
    let filename = "pt_mstr.json";
    let fullpath = file_path.join(filename);
    let datas = if is_cache_valid(&fullpath, None).await? {
        info!("Cache hit! Reading data from {:?}", fullpath);
        let json_content = fs::read_to_string(&fullpath).await?;
        serde_json::from_str(&json_content)?
    } else {
        info!("Cache missed or expired. Querying database...");
        let db_datas: Vec<PtMstr> = sqlx::query_as::<_, PtMstr>(
            r#"
            SELECT  
            id,
            pt_part,
            CONCAT(pt_desc1, ' ', pt_desc2) as 'desc',
            pt_dsgn_grp,
            pt_um,
            pt_buyer,
            pt_status,
            pt_size,
            pt_vend,
            pt_pm_code,
            pt_drwg_size,
            pt__log02 AS 'floorstock',
            pt_sfty_stk,
            pt_pur_lead,
            pt_mfg_lead,
            pt_insp_lead,
            pt_prod_line,
            pt_part_type,
            pt_group,
            pt_promo,
            pt_cum_lead,
            pt_draw,
            pt_drwg_loc,
            pt__log01 AS 'need_isir',
            pt__chr01 AS 'status_isir',
            pt__chr03 AS 'spsd_control',
            pt_ord_min,
            pt_iss_pol,
            pt_loc,
            pt_taxable,
            pt_taxc,
            pt_shelflife,
            pt_yield_pct,
            pt__chr05 AS 'exp_lkl_oem',
            pt__chr06 AS 'prod_category',
            pt__chr07 AS 'construction',
            pt__chr08 AS 'finishing',
            pt_added,
            pt__chr09 AS 'project',
            pt_rev,
            pt_ms,
            pt_abc,
            pt_lot_ser,
            pt_rctpo_status,
            pt_rctpo_active,
            pt_rctwo_status,
            pt_rctwo_active,
            pt_sfty_time,
            pt_article,
            pt_ord_qty,
            pt_site
            FROM dbqad_live.pt_mstr
            "#,
        )
        .fetch_all(pool)
        .await?;
        save_as_json(&db_datas, file_path, filename).await?;
        db_datas
    };
    let query_duration = query_start.elapsed();
    info!("Fetch Demand data running duration: {:?}", query_duration);
    Ok(datas)
}
