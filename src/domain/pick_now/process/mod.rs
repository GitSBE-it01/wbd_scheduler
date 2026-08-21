use crate::core::{error::AppResult, state::AppState};
use crate::domain::pick_now::{cek_past_id, demand, inventory, model::Record, pic};
pub mod caching;
mod delete;
mod inserts;
use crate::domain::qad_data::pt_mstr::ItemMstr;
use rust_decimal::Decimal;
use time::{OffsetDateTime, Weekday};
use tracing::info;

pub async fn run(state: &AppState) -> AppResult<()> {
    info!("-->> Fetching data for process");
    let (past_result, pic, dmd, inv, pt_mstr) = tokio::try_join!(
        cek_past_id::run(state),
        pic::run(state),
        demand::run(state),
        inventory::run(state),
        ItemMstr::get(state)
    )?;
    info!(
        past_result_count = past_result.len(),
        pic_count = pic.len(),
        demand_count = dmd.len(),
        inv_count = inv.len(),
        pt_mstr_count = pt_mstr.len(),
        "-->> Finished fetching batch data"
    );
    let mut records: Vec<Record> = Vec::with_capacity(dmd.len() + inv.len());
    records.extend(dmd.into_iter().map(Record::Demand));
    records.extend(inv.into_iter().map(Record::Inventory));
    records.sort_by(|a, b| (a.dept(), a.item(), a._date()).cmp(&(b.dept(), b.item(), b._date())));
    let mut running_balance = Decimal::ZERO;
    let mut computed_results = Vec::with_capacity(records.len());
    let mut current_group: Option<(String, String)> = None;
    info!("-->> Main Processing Pick Now Record");
    for record in records {
        let item = record.item().to_string();
        let dept = record.dept().to_string();
        let id = record.lot_id().to_string();
        let key = (item.clone(), dept.clone());
        if current_group.as_ref() != Some(&key) {
            running_balance = Decimal::ZERO; // 🔄 Reset running total for the new group
            current_group = Some(key);
        }
        let delta = match &record {
            Record::Demand(_) => -record.qty(), // ✅ Fixed: Demands decrease balance (-)
            Record::Inventory(_) => record.qty_oh(), // ✅ Inventory increases balance (+)
        };
        running_balance += delta;
        let now = OffsetDateTime::now_local()
            .unwrap_or_else(|_| OffsetDateTime::now_utc())
            .date();
        let current_day = now.weekday();
        let exists = if matches!(current_day, Weekday::Sunday | Weekday::Monday) {
            !past_result
                .iter()
                .any(|dtx| dtx.wo_id == id && dtx.id_new == "new")
        } else {
            // Normal check for other days
            past_result.iter().any(|dtx| dtx.wo_id == id)
        };
        let new_val = match &record {
            Record::Demand(_) => {
                if exists == false {
                    "new".to_string()
                } else {
                    String::new()
                }
            }
            Record::Inventory(_) => String::new(), // ✅ Inventory increases balance (+)
        };
        let val_acc = running_balance.min(Decimal::ZERO);
        let pick: String = if val_acc < Decimal::ZERO {
            "Pick".to_string()
        } else {
            String::new()
        };
        let desc1: Option<String> = pt_mstr
            .iter()
            .find(|dtx| dtx.pt_part == item)
            .map(|dtx| dtx.desc.clone());
        let desc2: Option<String> = record.item_id().and_then(|id| {
            pt_mstr
                .iter()
                .find(|dtx| dtx.pt_part == id)
                .map(|dtx| dtx.desc.clone())
        });
        let item_lower = item.to_lowercase();
        let opt = pic
            .iter()
            .find(|r| r.tipe.to_lowercase().contains(&item_lower))
            .map(|found| found.pic.clone());
        computed_results.push(Record::new(
            record, val_acc, pick, new_val, opt, desc1, desc2,
        ));
    }
    info!(
        process_data = computed_results.len(),
        "-->> Finish Processing data"
    );
    inserts::run(state, computed_results).await?;
    delete::run(state).await?;
    Ok(())
}

/*
use crate::{
    app_error::AppResult,
    app_state::AppState,
    app_utilities::{save_file::save_as_json, validation::is_cache_valid},
};
use std::path::Path;
use std::time::Instant;
use tracing::info;
mod cek_past_id;
mod demand;
mod inventory;
pub mod model;
mod pic;
use model::{Combine, Record};
use rust_decimal::Decimal;
use time::{OffsetDateTime, Weekday};

pub async fn run(state: &AppState) -> AppResult<()> {
    info!("Running Pick Now Service...");
    let total_start = Instant::now();
    info!("Checking current data...");
    let has_data: bool = sqlx::query_as::<_, Combine>(
        r#"
        SELECT * FROM dbpick_now.result_fix WHERE data_date = CURDATE()
        LIMIT 1
        "#,
    )
    .fetch_optional(state.pool())
    .await?
    .is_some();
    if has_data == true {
        info!("Pick Now is already running");
    } else {
        let (past_result, pic, dmd, inv) = tokio::try_join!(
            cek_past_id::run(state),
            pic::run(state),
            demand::run(state),
            inventory::run(state)
        )?;
        let mut records: Vec<Record> = Vec::with_capacity(dmd.len() + inv.len());
        records.extend(dmd.into_iter().map(Record::Demand));
        records.extend(inv.into_iter().map(Record::Inventory));
        records
            .sort_by(|a, b| (a.dept(), a.item(), a._date()).cmp(&(b.dept(), b.item(), b._date())));
        let fullpath = Path::new("./cache/rs_result.json");
        let file_path = Path::new("./cache/");
        // let result: Vec<Combine> = if is_cache_valid(&fullpath, ttl).await? {
        if is_cache_valid(&fullpath, Some(0)).await? {
            info!("rs_result is already exist");
        } else {
            let mut running_balance = Decimal::ZERO;
            let mut computed_results = Vec::with_capacity(records.len());
            let mut current_group: Option<(String, String)> = None;
            for record in records {
                let item = record.item().to_string();
                let dept = record.dept().to_string();
                let id = record.lot_id().to_string();
                let key = (item.clone(), dept.clone());
                if current_group.as_ref() != Some(&key) {
                    running_balance = Decimal::ZERO; // 🔄 Reset running total for the new group
                    current_group = Some(key);
                }
                let delta = match &record {
                    Record::Demand(_) => -record.qty(), // ✅ Fixed: Demands decrease balance (-)
                    Record::Inventory(_) => record.qty_oh(), // ✅ Inventory increases balance (+)
                };
                running_balance += delta;
                let now = OffsetDateTime::now_local()
                    .unwrap_or_else(|_| OffsetDateTime::now_utc())
                    .date();
                let current_day = now.weekday();
                let exists = if matches!(current_day, Weekday::Sunday | Weekday::Monday) {
                    // Returns false if found, true if not found
                    !past_result
                        .iter()
                        .any(|dtx| dtx.wo_id == id && dtx.id_new == "new")
                } else {
                    // Normal check for other days
                    past_result.iter().any(|dtx| dtx.wo_id == id)
                };
                // let exists: bool = past_result.iter().any(|dtx| dtx.wo_id == id);
                let new_val = if exists == false {
                    "new".to_string()
                } else {
                    String::new()
                };
                let val_acc = running_balance.min(Decimal::ZERO);
                let pick: String = if val_acc < Decimal::ZERO {
                    "Pick".to_string()
                } else {
                    String::new()
                };
                let opt = pic
                    .iter()
                    .find(|r| r.tipe == item)
                    .map(|found| found.pic.clone());
                // ✅ Fixed: Construct Combine struct directly
                computed_results.push(Record::new(record, val_acc, pick, new_val, opt));
            }
            save_as_json(&computed_results, file_path, "rs_result.json").await?;
            // save_as_csv(&computed_results, file_path, "rs_result.csv").await?;
        };

        let del_past = sqlx::query(
            "DELETE FROM dbpick_now.result_fix WHERE data_date = CURDATE() - INTERVAL 3 DAY",
        )
        .execute(state.pool())
        .await?;
        let rows_deleted = del_past.rows_affected();
        info!("Deleted past data for {} rows", rows_deleted);
    }
    let total_dur = total_start.elapsed();
    info!("Process Pick Now Scheduler END within {:?}", total_dur);
    Ok(())
}
*/
