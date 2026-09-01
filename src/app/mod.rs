pub mod error;
pub mod pick_now;
use serde::{Deserialize, Serialize};
// pub mod drawing_catalog;
// pub mod new_type_report;
// use crate::system::config::AppConfig;
// use crate::system::state::AppState;
pub use error::AppError;
// use std::time::Instant;
// use tracing::info;

pub async fn main_handler() -> Result<(), AppError> {
    todo!()
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
    use crate::system::config::AppConfig;
    use crate::system::io::csv::load_csv_data;
    use crate::system::state::AppState;
    use error::AppError;

    #[tokio::test]
    async fn test_main_run() -> Result<(), AppError> {
        let config = AppConfig::new(".env")?;
        let state = AppState::new(config).await?;
        Ok(())
    }

    use std::path::Path;
    #[tokio::test]
    async fn test_pick_now_handler() -> Result<(), AppError> {
        let file_path =
            Path::new("//192.168.0.203/QAD Daily Download/260831 - work_order_browse.csv");
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
            .collect::<Vec<_>>();
        Ok(())
    }
}
