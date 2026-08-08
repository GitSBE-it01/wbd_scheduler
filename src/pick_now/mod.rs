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
use model::Record;
use rust_decimal::Decimal;
// use time::Duration;
// use tokio::fs;

pub async fn run(state: &AppState) -> AppResult<()> {
    info!("Running Pick Now Service...");
    let total_start = Instant::now();
    let (past_result, pic, dmd, inv) = tokio::try_join!(
        cek_past_id::run(state),
        pic::run(state),
        demand::run(state),
        inventory::run(state)
    )?;
    let mut records: Vec<Record> = Vec::with_capacity(dmd.len() + inv.len());
    records.extend(dmd.into_iter().map(Record::Demand));
    records.extend(inv.into_iter().map(Record::Inventory));
    records.sort_by(|a, b| (a.dept(), a.item(), a._date()).cmp(&(b.dept(), b.item(), b._date())));
    let fullpath = Path::new("./cache/rs_result.csv");
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
            let exists: bool = past_result.iter().any(|dtx| dtx.wo_id == id);
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
    };
    let total_dur = total_start.elapsed();
    info!("Pick Now Total duration: {:?}", total_dur);
    Ok(())
}
