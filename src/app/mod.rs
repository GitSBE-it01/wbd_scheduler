pub mod error;
pub mod pick_now;
use crate::system::{config::AppConfig, state::AppState};
use serde::{Deserialize, Serialize};
// pub mod drawing_catalog;
// pub mod new_type_report;
// use crate::system::config::AppConfig;
// use crate::system::state::AppState;
pub use error::AppError;
use std::time::Instant;
use tracing::info;

pub async fn main_handler() -> Result<(), AppError> {
    info!("Starting Schedule... ");
    let config = AppConfig::new(".env")?;
    let state = AppState::new(config).await?;
    info!("Running Drawing Catalog... ");
    let mut time_record = Instant::now();
    // pick_now::handler(&state).await?;
    let mut elapsed = time_record.elapsed();
    info!(duration = ?elapsed, "Finished Drawing Catalog... ");
    info!("Running Pick Now... ");
    time_record = Instant::now();
    pick_now::handler(&state).await?;
    elapsed = time_record.elapsed();
    info!(duration = ?elapsed, "Finished Pick Now... ");
    info!("Finished Scheduler... ");
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WoBrowseRecord {
    pub item_number: String,
    pub desc1: String,
    pub desc2: String,
    pub p_m: String,
    pub supplier: String,
    pub site: String,
    pub work_order: String,
    pub id: String,
    pub qty_ord: String,
    pub qty_open: String,
    pub qty_comp: String,
    pub rel_date: String,
    pub ord_date: String,
    pub due_date: String,
    pub lot_serial: String,
    pub remarks: String,
    pub sales_job: String,
    pub wo_stat: String,
    pub close_date: String,
    pub wo_close_by: String,
    pub acc_close: String,
    pub assy_line: String,
    pub routing: String,
    pub fixed_r: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::pick_now::model::Record;
    use crate::system::cache;
    use crate::system::db::DatabaseCacheable;
    use crate::system::config::AppConfig;
    use crate::system::db::{
        pn_demand::PnDemand, pn_dept_inv::PnInv, pn_past_id::PnPastID, pn_pic::PnPic,
        pn_result::PnResult, qad_item::QadItemMstr,
    };
    use crate::system::io::csv::{load_csv_data, save_as_csv};
    use crate::system::state::AppState;
    use error::AppError;
    use rust_decimal::Decimal;
    use time::{OffsetDateTime, Weekday};
    #[tokio::test]
    async fn test_cache() -> Result<(), AppError> {
        let config = AppConfig::new(".env")?;
        let state = AppState::new(config).await?;
        let dmd: Vec<PnPic> = PnPic::fetch_db(&state).await?;
        let store = cache::CacheEngine::new("./cache");
        // store.put(&"pn_pic", &dmd).expect("Failed to write cache");
        Ok(())
    }

    #[tokio::test]
    async fn test_dmd() -> Result<(), AppError> {
        let config = AppConfig::new(".env")?;
        let state = AppState::new(config).await?;
        let dmd: Vec<PnDemand> = PnDemand::fetch_db(&state).await?;
        save_as_csv(&dmd, Path::new("./cache/"), "pn_demand.csv").await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_pn() -> Result<(), AppError> {
        let config = AppConfig::new(".env")?;
        let state = AppState::new(config).await?;
        let (past_result, pic, dmd, inv, pt_mstr) = tokio::try_join!(
            PnPastID::fetch_db(&state),
            PnPic::fetch_db(&state),
            PnDemand::fetch_db(&state),
            PnInv::fetch_db(&state),
            QadItemMstr::fetch_db(&state)
        )?;
        save_as_csv(&past_result, Path::new("./cache/"), "past_result.csv").await?;
        println!(
            "Fetched {} past_result, {} pic, {} dmd, {} inv, {} pt_mstr",
            past_result.len(),
            pic.len(),
            dmd.len(),
            inv.len(),
            pt_mstr.len()
        );
        let mut records: Vec<Record> = Vec::with_capacity(dmd.len() + inv.len());
        records.extend(dmd.into_iter().map(Record::Demand));
        records.extend(inv.into_iter().map(Record::Inventory));
        records
            .sort_by(|a, b| (a.dept(), a.item(), a._date()).cmp(&(b.dept(), b.item(), b._date())));
        let mut running_balance = Decimal::ZERO;
        let mut transform_to_result: Vec<PnResult> = Vec::with_capacity(records.len());
        let mut current_group: Option<(String, String)> = None;
        let mut cek_new = 0;
        for record in records {
            let item = record.item().to_string();
            let dept = record.dept().to_string();
            let id = record.lot_id().to_string();
            let key = (item.clone(), dept.clone());
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
            let row = match record {
                Record::Demand(d) => PnResult {
                    item: d.item,
                    _desc: desc1,
                    remark: "2.demand".to_string(),
                    lot__id: Some(d.lot_id),
                    loc__line: Some(d.loc_line),
                    dept: d.dept,
                    qty: Decimal::from_f64_retain(d.qty).unwrap_or(Decimal::ZERO),
                    valAcc: Decimal::ZERO,
                    qty_OH: Decimal::ZERO,
                    lotOH: None,
                    _date: d.rel_dt,
                    rel_dt: d.rel_dt,
                    due_dt: d.due_dt,
                    wo_rmks: Some(d.wo_rmks),
                    pic: opt,
                    pick: String::new(),
                    id_new: String::new(),
                    item_id: Some(d.item_id),
                    _desc2: desc2,
                    old_id: Some(d.old_id),
                },
                Record::Inventory(i) => PnResult {
                    item: i.item,
                    _desc: desc1,
                    remark: "1.on hand".to_string(),
                    lot__id: None,
                    loc__line: None,
                    dept: i.dept,
                    qty: Decimal::ZERO,
                    valAcc: Decimal::ZERO,
                    qty_OH: Decimal::from_f64_retain(i.qty_oh).unwrap_or(Decimal::ZERO),
                    lotOH: Some(i.lot_oh),
                    _date: i._date,
                    rel_dt: i._date,
                    due_dt: None,
                    wo_rmks: None,
                    pic: opt,
                    pick: String::new(),
                    id_new: String::new(),
                    item_id: None,
                    old_id: None,
                    _desc2: desc2,
                },
            };
            transform_to_result.push(row);
        }
        save_as_csv(
            &transform_to_result,
            Path::new("./cache/"),
            "transform_to_result.csv",
        )
        .await?;
        transform_to_result
            .sort_by(|a, b| (&a.dept, &a.item, &a._date).cmp(&(&b.dept, &b.item, &b._date)));
        let mut computed_results: Vec<PnResult> = Vec::with_capacity(transform_to_result.len());
        for record in transform_to_result {
            let key = (record.item.clone(), record.dept.clone());
            if current_group.as_ref() != Some(&key) {
                running_balance = Decimal::ZERO; // 🔄 Reset running total for the new group
                current_group = Some(key);
            }
            let delta = if &record.remark == "2.demand" {
                -record.qty // ✅ Demands decrease balance (-)
            } else {
                record.qty_OH // ✅ Inventory increases balance (+)
            };
            running_balance += delta;
            let now = OffsetDateTime::now_local()
                .unwrap_or_else(|_| OffsetDateTime::now_utc())
                .date();
            let current_day = now.weekday();
            let curr_id_exist_in_past = record
                .lot__id
                .as_ref()
                .is_some_and(|lot_id| past_result.iter().any(|dtx| &dtx.wo_id == lot_id));
            let curr_id_is_new_in_past = record.lot__id.as_ref().is_some_and(|lot_id| {
                past_result
                    .iter()
                    .any(|dtx| &dtx.wo_id == lot_id && dtx.id_new == "new")
            });
            // let curr_id_exist_in_past = past_result.iter().any(|dtx| dtx.wo_id == id);
            // let curr_id_is_new_in_past = past_result
            //     .iter()
            //     .any(|dtx| dtx.wo_id == id && dtx.id_new == "new");
            let exists: bool = if matches!(
                current_day,
                Weekday::Saturday | Weekday::Sunday | Weekday::Monday
            ) {
                if curr_id_exist_in_past {
                    if curr_id_is_new_in_past { true } else { false }
                } else {
                    true
                }
            } else {
                if curr_id_exist_in_past { false } else { true }
            };
            let new_val = if &record.remark == "2.demand" {
                if exists == true {
                    "new".to_string()
                } else {
                    String::new()
                }
            } else {
                String::new()
            };
            if new_val == "new" {
                cek_new += 1;
            }
            let val_acc = running_balance.min(Decimal::ZERO);
            let pick: String = if val_acc < Decimal::ZERO {
                "Pick".to_string()
            } else {
                String::new()
            };

            let row = PnResult {
                item: record.item,
                _desc: record._desc,
                remark: record.remark,
                lot__id: record.lot__id,
                loc__line: record.loc__line,
                dept: record.dept,
                qty: record.qty,
                valAcc: val_acc,
                qty_OH: record.qty_OH,
                lotOH: record.lotOH,
                _date: record._date,
                rel_dt: record.rel_dt,
                due_dt: record.due_dt,
                wo_rmks: record.wo_rmks,
                pic: record.pic,
                pick: pick,
                id_new: new_val,
                item_id: record.item_id,
                _desc2: record._desc2,
                old_id: record.old_id,
            };
            computed_results.push(row);
        }
        save_as_csv(
            &computed_results,
            Path::new("./cache/"),
            "computed_results.csv",
        )
        .await?;
        let mut count_new = 0;
        let mut count_service_data = 0;
        let mut count_service_data_new = 0;
        let mut service_id: Vec<String> = Vec::new();
        computed_results.iter().for_each(|r| {
            if r.id_new == "new" {
                count_new += 1;
            }
            if r.dept == "QA" {
                if let Some(lot_id) = &r.lot__id {
                    if !service_id.contains(lot_id) {
                        service_id.push(lot_id.clone());
                    }
                }
                count_service_data += 1;
                if r.id_new == "new" {
                    count_service_data_new += 1;
                    // println!(
                    //     "Service Data Ne1: item = {}, lot_id = {}",
                    //     r.item,
                    //     r.lot__id.as_deref().unwrap_or("N/A")
                    // );
                }
            }
        });
        println!(
            "recek_new_calc = {}, Cek New = {}, cek service data = {}, cek service data new = {}",
            cek_new, count_new, count_service_data, count_service_data_new,
        );
        PnResult::insert(&state, &computed_results).await?;
        Ok(())
    }

    #[tokio::test]
    async fn ts_clock() -> Result<(), AppError> {
        let clock = crate::system::state::clock::AppClock::new();
        println!(
            "Current timestamp: {}",
            clock.date_stringfy(-1, Some("[year repr:last_two][month][day]"))
        );
        Ok(())
    }

    use std::path::Path;
    #[tokio::test]
    async fn ts_qad_csv_read() -> Result<(), AppError> {
        let clock = crate::system::state::clock::AppClock::new();
        let clock_date_data = clock.date_stringfy(-1, Some("[year repr:last_two][month][day]"));
        println!("cek clock date data : {}", clock_date_data);
        let new_file = format!(
            "//192.168.0.203/QAD Daily Download/{} - work_order_browse.csv",
            clock_date_data
        );
        let file_path = Path::new(&new_file);
        let hd = [
            "item_number",
            "desc1",
            "desc2",
            "p_m",
            "supplier",
            "site",
            "work_order",
            "id",
            "qty_ord",
            "qty_open",
            "qty_comp",
            "rel_date",
            "ord_date",
            "due_date",
            "lot_serial",
            "remarks",
            "sales_job",
            "wo_stat",
            "close_date",
            "wo_close_by",
            "acc_close",
            "assy_line",
            "routing",
            "fixed_r",
        ];
        let records: Vec<WoBrowseRecord> = load_csv_data(file_path, &hd)?;
        println!("Loaded {} records from CSV", records.len());
        let fltr_records = records
            .into_iter()
            .filter(|r| r.wo_stat == "R")
            .collect::<Vec<WoBrowseRecord>>();
        println!(
            "Filtered {} records from CSV for Stats R",
            fltr_records.len()
        );
        Ok(())
    }
}
