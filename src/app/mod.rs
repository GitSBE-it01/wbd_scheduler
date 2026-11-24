pub mod ebom_parent;
pub mod error;
pub mod pick_now;
use crate::system::{config::AppConfig, state::AppState};
use serde::{Deserialize, Serialize};
pub mod drawing_catalog;
pub mod item_dev;
pub mod jig_catalog;
pub mod new_type_report;
pub use error::AppError;
use std::time::Instant;
use tracing::info;

pub async fn main_handler() -> Result<(), AppError> {
    info!("Starting Schedule... ");
    let config = AppConfig::new(".env")?;
    let state = AppState::new(config).await?;
    info!("Running Drawing Catalog... ");
    let starts = Instant::now();
    let mut time_record = Instant::now();
    drawing_catalog::run_drawing_catalog(&state).await?;
    let mut elapsed = time_record.elapsed();
    info!(duration = ?elapsed, "Finished Drawing Catalog... ");
    info!("Running Jig Catalog... ");
    time_record = Instant::now();
    jig_catalog::run_jig_catalog(&state).await?;
    elapsed = time_record.elapsed();
    info!(duration = ?elapsed, "Finished Jig Catalog... ");
    info!("Running Pick Now... ");
    time_record = Instant::now();
    pick_now::run_pick_now(&state).await?;
    elapsed = time_record.elapsed();
    info!(duration = ?elapsed, "Finished Pick Now... ");
    info!("Running New Type Report... ");
    time_record = Instant::now();
    new_type_report::run_ntr(&state).await?;
    elapsed = time_record.elapsed();
    info!(duration = ?elapsed, "Finished New Type Report... ");
    info!("Running Item Dev Scheduler... ");
    time_record = Instant::now();
    item_dev::run_item_dev(&state).await?;
    elapsed = time_record.elapsed();
    info!(duration = ?elapsed, "Finished Item Dev Scheduler... ");
    info!("Running eBOM Parent... ");
    time_record = Instant::now();
    ebom_parent::run_ebom(&state).await?;
    elapsed = time_record.elapsed();
    info!(duration = ?elapsed, "Finished eBOM Parent... ");
    let total_elapsed = starts.elapsed();
    info!(total_duration = ?total_elapsed,"Finished Scheduler... ");
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
    use crate::system::config::AppConfig;
    use crate::system::db::DatabaseCacheable;
    use crate::system::db::{PnDemand, PnInv, PnPastID, PnPic, PnResult, QadItemMstr};
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
