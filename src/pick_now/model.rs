use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::Date;
time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CekID {
    pub wo_id: String,
    pub id_new: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Demand {
    pub item: String,
    pub lot_id: String,
    pub loc_line: String,
    pub dept: String,
    pub qty: f64,
    #[serde(with = "date_format::option")]
    pub _date: Option<Date>,
    #[serde(with = "date_format::option")]
    pub rel_dt: Option<Date>,
    #[serde(with = "date_format::option")]
    pub due_dt: Option<Date>,
    pub wo_rmks: String,
    pub item_id: String,
    pub old_id: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Inv {
    pub item: String,
    pub dept: String,
    pub qty_oh: f64,
    #[serde(with = "date_format::option")]
    pub _date: Option<Date>,
    pub lot_oh: String,
}

// Wrap both types in an Enum
#[derive(Debug, Clone)]
pub enum Record {
    Demand(Demand),
    Inventory(Inv),
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
    pub fn qty(&self) -> f64 {
        match self {
            Record::Demand(d) => d.qty,
            Record::Inventory(_) => 0.00,
        }
    }
    pub fn qty_oh(&self) -> f64 {
        match self {
            Record::Demand(_) => 0.00,
            Record::Inventory(i) => i.qty_oh,
        }
    }
    pub fn new(self, qty_val: f64, pck: String, inew: String) -> Combine {
        match self {
            Record::Demand(d) => Combine {
                item: d.item,
                lot_id: Some(d.lot_id),
                loc_line: Some(d.loc_line),
                dept: d.dept,
                qty: d.qty,
                qty_oh: 0.00,
                _date: d.rel_dt,
                rel_dt: d.rel_dt,
                due_dt: d.due_dt,
                wo_rmks: Some(d.wo_rmks),
                pick: pck,
                id_new: inew,
                item_id: Some(d.item_id),
                old_id: Some(d.old_id),
                lot_oh: None,
                remark: "2.demand".to_string(),
                val_acc: qty_val,
            },
            Record::Inventory(i) => Combine {
                item: i.item,
                lot_id: None,
                loc_line: None,
                dept: i.dept,
                qty: 0.00,
                qty_oh: i.qty_oh,
                _date: i._date,
                rel_dt: i._date,
                due_dt: None,
                wo_rmks: None,
                pick: pck,
                id_new: inew,
                item_id: None,
                old_id: None,
                lot_oh: Some(i.lot_oh),
                remark: "1.on hand".to_string(),
                val_acc: qty_val,
            },
        }
    }
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Combine {
    pub item: String,
    pub lot_id: Option<String>,
    pub loc_line: Option<String>,
    pub dept: String,
    pub remark: String,
    pub qty: f64,
    pub val_acc: f64,
    pub qty_oh: f64,
    #[serde(with = "date_format::option")]
    pub _date: Option<Date>,
    #[serde(with = "date_format::option")]
    pub rel_dt: Option<Date>,
    #[serde(with = "date_format::option")]
    pub due_dt: Option<Date>,
    pub pick: String,
    pub id_new: String,
    pub wo_rmks: Option<String>,
    pub item_id: Option<String>,
    pub old_id: Option<String>,
    pub lot_oh: Option<String>,
}
