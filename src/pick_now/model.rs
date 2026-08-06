use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::Date;
time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Debug, FromRow, Serialize, Deserialize)]
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
/*
#[derive(Debug, FromRow)]
pub struct Inv {
    pub item: String,
    pub dept: String,
    pub qty_oh: f64,
    pub _date: Date,
    pub lot_oh: String,
}

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
