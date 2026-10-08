use crate::system::db::{PnDemand, PnInv};
use rust_decimal::Decimal;
use time::Date;
pub struct PnSummary {
    pub computed_item: usize,
    pub items_inserted: usize,
}
pub enum PnOutcome {
    Skipped { reason: String },
    Completed(PnSummary),
}
#[derive(Debug, Clone)]
pub enum Record {
    Demand(PnDemand),
    Inventory(PnInv),
}

impl Record {
    pub fn item(&self) -> &str {
        match self {
            Record::Demand(d) => &d.item,
            Record::Inventory(i) => &i.item,
        }
    }
    pub fn lot_id(&self) -> &str {
        match self {
            Record::Demand(d) => &d.lot_id,
            Record::Inventory(_) => "",
        }
    }
    pub fn dept(&self) -> &str {
        match self {
            Record::Demand(d) => &d.dept,
            Record::Inventory(i) => &i.dept,
        }
    }
    pub fn _date(&self) -> Option<Date> {
        match self {
            Record::Demand(d) => d._date,
            Record::Inventory(i) => i._date,
        }
    }
    pub fn qty(&self) -> Decimal {
        match self {
            Record::Demand(d) => Decimal::from_f64_retain(d.qty).unwrap_or(Decimal::ZERO),
            Record::Inventory(_) => Decimal::ZERO,
        }
    }
    pub fn item_id(&self) -> Option<&str> {
        match self {
            Record::Demand(d) => Some(&d.item_id),
            Record::Inventory(_) => None,
        }
    }
    pub fn qty_oh(&self) -> Decimal {
        match self {
            Record::Demand(_) => Decimal::ZERO,
            Record::Inventory(i) => Decimal::from_f64_retain(i.qty_oh).unwrap_or(Decimal::ZERO),
        }
    }
}
