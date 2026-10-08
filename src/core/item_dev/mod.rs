use time::Date;
pub mod error;
use crate::system::cache;
use crate::system::db::{
    DcDrawing, ItemDevDetail, ItemDevMaster,
    item_dev::master::{CostStatus, DrawingStatus, ItemDevStatus},
};
use crate::system::state::AppState;
pub use error::ItemDevError;

pub struct ItemDevSummary {
    pub records_count: usize,
    pub records_insert: usize,
}
pub async fn item_dev_handler(state: &AppState) -> Result<ItemDevSummary, ItemDevError> {
    let store = cache::CacheEngine::new("./cache");
    let records = store
        .db_handler::<ItemDevDetail>(state, "item_dev_detail".to_string())
        .await?;
    let drwg = store
        .db_handler::<DcDrawing>(state, "cache_data__drawing".to_string())
        .await?;
    let process_records: Vec<ItemDevMaster> = Vec::new();
    for record in records.data {
        let mut status = ItemDevStatus::Open;
        let cost: CostStatus = match record.cost_cek {
            false => match record.pm_code.as_str() {
                "P" => CostStatus::Purchasing,
                "D" => CostStatus::SBE4,
                "M" => {
                    if record.buyer == "" {
                        CostStatus::Purchasing
                    } else {
                        CostStatus::NoCost
                    }
                }
            },
            true => {
                if record.bom_cek && record.routing_cek {
                    status = ItemDevStatus::Close
                };
                CostStatus::Ok
            }
        };
        let drawing = drwg
            .data
            .iter()
            .find(|dt| dt.item == record.item_number)
            .and_then(|dt| dt.drawing_path.as_ref()) // Turns Option<PathBuf> into Option<&PathBuf>
            .map(|path| DrawingStatus::Ok(path.to_string_lossy().into_owned())) // Converts PathBuf to String
            .unwrap_or(DrawingStatus::NoData);
        process_records.push(ItemDevMaster {
            item: record.item_number.clone(),
            bom: record.bom_cek.clone(),
            routing: record.routing_cek.clone(),
            cost,
            drawing,
            status,
        })
    }
    let records_count = process_records.len();
    Ok(ItemDevSummary {
        records_count,
        records_insert: 0,
    })
}
