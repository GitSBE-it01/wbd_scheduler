use super::ItemDevError;
use crate::system::cache;
use crate::system::db::{
    DcDrawing,
    item_dev::isir::ItemDevIsir,
    item_dev::master::DBItemDevMstr,
    item_dev::reff::ItemDevException,
    item_dev::result::{CostStatus, ItemDevMaster, ItemDevStatus},
    qad::{bom::QadPsMstr, cost::QadPriceList, item::QadItemMstr, routing::QadRouting},
};
use crate::system::state::AppState;
use time::Date;
use tracing::info;

#[derive(Debug, Clone)]
pub struct ItemDevMstrSummary {
    pub records_count: usize,
    pub records_insert: usize,
}
pub async fn item_dev_master_records(state: &AppState) -> Result<ItemDevMstrSummary, ItemDevError> {
    let store = cache::CacheEngine::new("./cache");
    let (qad_item, qad_bom, qad_route, qad_pl) = tokio::try_join!(
        store.db_handler::<QadItemMstr>(state, "items".to_string()),
        store.db_handler::<QadPsMstr>(state, "product_structure".to_string()),
        store.db_handler::<QadRouting>(state, "routing".to_string()),
        store.db_handler::<QadPriceList>(state, "price_list".to_string()),
    )?;
    let (isir, reff) = tokio::try_join!(
        store.db_handler::<ItemDevIsir>(state, "items".to_string()),
        store.db_handler::<ItemDevException>(state, "product_structure".to_string()),
    )?;
    let mut records: Vec<ItemDevMaster> = Vec::new();
    info!("Processing Records...");
    for rec in qad_item.data {
        if matches!(rec.pt_status.as_deref(), Some("SPSD" | "OBSOLETE" | "MPSD"))
            || matches!(rec.pt_buyer.as_deref(), Some("N-TAGIH" | "N-Tagih"))
        {
            continue;
        }
        let bom: bool = qad_bom.data.iter().any(|dtx| dtx.ps_par == rec.pt_part);
        let routing: bool = qad_route
            .data
            .iter()
            .any(|dtx| dtx.ro_routing == rec.pt_part);
        let cost_cek: bool = qad_pl.data.iter().any(|dtx| dtx.pc_part == rec.pt_part);
        let cost: CostStatus = match cost_cek {
            false => match rec.pt_pm_code.as_deref() {
                Some("P") => CostStatus::Purchasing,
                Some("D") => CostStatus::SBE4,
                _ => {
                    if rec.pt_buyer.as_deref() == Some("S-SUB")
                        || rec.pt_buyer.as_deref() == Some("S-SUBCON")
                    {
                        CostStatus::Purchasing
                    } else {
                        CostStatus::NoCost
                    }
                }
            },
            true => CostStatus::Ok,
        };
        let status = match rec.pt_status.as_deref() {
            Some("DEVELOP") => ItemDevStatus::Open,
            _ => {
                if rec.need_isir.as_deref() == Some("no") {
                    ItemDevStatus::Close
                } else {
                    if rec.status_isir.as_deref() == Some("PASS") {
                        ItemDevStatus::Close
                    } else {
                        ItemDevStatus::Open
                    }
                }
            }
        };
        let (no_srf, no_sample) = isir
            .data
            .iter()
            .find(|dt| dt.partrelease.as_deref() == Some(rec.pt_part.as_str())) // Matched as Option<&str>
            .map(|dx| (dx.id_srf, dx.id_sample))
            .unwrap_or_else(|| (0, Some(0)));
        records.push(ItemDevMaster {
            item: rec.pt_part.clone(),
            bom,
            routing,
            cost,
            status,
            no_srf,
            no_sample,
        })
    }
    let records_count = records.len();
    info!(record_count = ?records_count, "Done Processing Records...");
    let records_insert: usize = ItemDevMaster::upsert(state, &records)
        .await?
        .try_into()
        .unwrap();
    Ok(ItemDevMstrSummary {
        records_count,
        records_insert,
    })
}
