use crate::core::pick_now::error::PnError;
use crate::core::pick_now::model::Record;
use crate::system::db::{
    pn_demand::PnDemand, pn_dept_inv::PnInv, pn_pic::PnPic, pn_result::PnResult,
    qad_item::QadItemMstr,
};
use rust_decimal::Decimal;

pub fn collect_new_records(
    demand: Vec<PnDemand>,
    inventory: Vec<PnInv>,
    pic: Vec<PnPic>,
    pt_mstr: Vec<QadItemMstr>,
) -> Result<Vec<PnResult>, PnError> {
    let mut records: Vec<Record> = Vec::with_capacity(demand.len() + inventory.len());
    records.extend(demand.into_iter().map(Record::Demand));
    records.extend(inventory.into_iter().map(Record::Inventory));
    records.sort_by(|a, b| (a.dept(), a.item(), a._date()).cmp(&(b.dept(), b.item(), b._date())));
    let mut collect_record: Vec<PnResult> = Vec::with_capacity(records.len());
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
        collect_record.push(row);
    }
    Ok(collect_record)
}
