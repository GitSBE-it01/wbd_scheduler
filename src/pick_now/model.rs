use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::Date;
time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Demand {
    pub item: String,
    pub lot_id: String,
    pub loc_line: String,
    pub dept: String,
    pub qty: f64,
    // pub _date: Date,
    #[serde(with = "date_format::option")]
    pub rel_dt: Option<Date>,
    #[serde(with = "date_format::option")]
    pub due_dt: Option<Date>,
    pub wo_rmks: Option<String>,
    pub item_id: String,
    pub old_id: Option<String>,
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
    pub fn loc_line(&self) -> &str {
        match self {
            Record::Demand(d) => &d.loc_line,
            Record::Inventory(i) => &i.loc_line,
        }
    }
    pub fn dept(&self) -> &str {
        match self {
            Record::Demand(d) => &d.dept,
            Record::Inventory(i) => &i.dept,
        }
    }
    pub fn qty(&self) -> f64 {
        match self {
            Record::Demand(d) => d.qty,
            Record::Inventory(i) => i.qty_oh,
        }
    }
    pub fn _date(&self) -> Option<Date> {
        match self {
            Record::Demand(d) => d.rel_dt,
            Record::Inventory(i) => i._date,
        }
    }
    pub fn rel_dt(&self) -> Date {
        match self {
            Record::Demand(d) => d.rel_dt,
            Record::Inventory(i) => i.rel_dt,
        }
    }
    pub fn due_dt(&self) -> Date {
        match self {
            Record::Demand(d) => d.due_dt,
            Record::Inventory(i) => i.due_dt,
        }
    }
    pub fn wo_rmks(&self) -> Date {
        match self {
            Record::Demand(d) => d.wo_rmks,
            Record::Inventory(i) => i.wo_rmks,
        }
    }
    pub fn item_id(&self) -> Date {
        match self {
            Record::Demand(d) => d.item_id,
            Record::Inventory(i) => i.item_id,
        }
    }
    pub fn old_id(&self) -> Date {
        match self {
            Record::Demand(d) => d.old_id,
            Record::Inventory(i) => i.old_id,
        }
    }
    pub fn qty_oh(&self) -> Date {
        match self {
            Record::Demand(d) => d.qty_oh,
            Record::Inventory(i) => i.qty_oh,
        }
    }
    pub fn lot_oh(&self) -> Date {
        match self {
            Record::Demand(d) => d.lot_oh,
            Record::Inventory(i) => i.lot_oh,
        }
    }
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct combine {
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
    pub wo_rmks: Option<String>,
    pub item_id: String,
    pub old_id: Option<String>,
    pub qty_oh: f64,
    pub lot_oh: String,
}
/*
#[derive(Debug, FromRow)]
pub struct PickNow {
    data_date: Date,
    item: String,
    _desc: String,
    remark: String,
    lot_id: String,
    loc_line: String,
    dept: String,
    qty: f64,
    val_acc: String,
    qty_oh: f64,
    lot_oh: String,
    _date: Date,
    rel_dt: Date,
    due_dt: Date,
    wo_rmks: String,
    pic: String,
    pick: String,
    id_new: String,
    item_id: String,
    _desc2: String,
    old_id: String,
}
    */
