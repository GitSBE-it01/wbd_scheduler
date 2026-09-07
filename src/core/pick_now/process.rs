use crate::core::pick_now::error::PnError;
use crate::core::pick_now::model::Record;
use crate::system::db::{pn_past_id::PnPastID, pn_result::PnResult};
use rust_decimal::Decimal;
use time::{OffsetDateTime, Weekday};
pub fn pn_process(
    raw_record: Vec<PnResult>,
    past_result: Vec<PnPastID>,
) -> Result<Vec<PnResult>, PnError> {
    let mut records: Vec<PnResult> = raw_record;
    records.sort_by(|a, b| (&a.dept, &a.item, &a._date).cmp(&(&b.dept, &b.item, &b._date)));
    let mut computed_results: Vec<PnResult> = Vec::with_capacity(records.len());
    let mut running_balance = Decimal::ZERO;
    let mut current_group: Option<(String, String)> = None;
    for record in records {
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
    Ok(computed_results)
}
