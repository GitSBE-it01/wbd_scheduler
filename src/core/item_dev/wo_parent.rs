use super::ItemDevError;
use crate::app::ebom_parent;
use crate::system::cache;
use crate::system::db::{
    DcDrawing,
    ebom_parent::EBomParent,
    item_dev::isir::ItemDevIsir,
    item_dev::master::DBItemDevMstr,
    item_dev::parent_data::ItemDevParentData,
    item_dev::result::{ItemDevMaster, ItemDevStatus},
    item_dev::wo_bom::DBItemDevPrntRaw,
};
use crate::system::state::AppState;
use time::Date;
use tracing::info;

#[derive(Debug, Clone)]
pub struct ItemDevPrntSummary {
    pub records_count: usize,
    pub records_insert: usize,
}

pub async fn item_dev_wo_parent_check(
    state: &AppState,
) -> Result<ItemDevPrntSummary, ItemDevError> {
    let store = cache::CacheEngine::new("./cache");
    let (mstr_data, it_dev_bom) = tokio::try_join!(
        store.db_handler::<DBItemDevMstr>(state, "item_dev_master".to_string()),
        store.db_handler::<DBItemDevPrntRaw>(state, "it_dev_bom".to_string()),
    )?;
    let mut to_inserts: Vec<ItemDevParentData> = Vec::new();
    for rec in mstr_data.data {
        let dt_it_dev = it_dev_bom
            .data
            .iter()
            .filter(|dt| dt.item == rec.item)
            .cloned()
            .collect::<Vec<DBItemDevPrntRaw>>();
        let data_parent: Vec<String> = dt_it_dev
            .iter()
            .filter_map(|dt| dt.parent.as_deref()) // Unpacks Option<String> to Option<&str>
            .filter(|p| !p.is_empty()) // Drops empty strings if any exist
            .map(|p| format!("- {}", p))
            .collect();
        let parent_ls: Option<String> = if data_parent.is_empty() {
            None
        } else {
            Some(data_parent.join("\n"))
        };
        // 2. Find the FIRST item in the vector where these fields are NOT None
        let closest_wo_nbr = dt_it_dev.iter().find_map(|dt| dt.wo_nbr.clone());
        let closest_wo_lot = dt_it_dev.iter().find_map(|dt| dt.wo_lot.clone());
        let closest_rel_date = dt_it_dev.iter().find_map(|dt| dt.rel_date);
        to_inserts.push(ItemDevParentData {
            itd_master_id: rec.id,
            parent_ls,
            closest_wo_nbr,
            closest_wo_lot,
            closest_rel_date,
        });
    }
    let records_count = to_inserts.len();
    let records_insert: usize = ItemDevParentData::inserts(state, to_inserts)
        .await?
        .try_into()
        .unwrap();
    Ok(ItemDevPrntSummary {
        records_count,
        records_insert,
    })
}
