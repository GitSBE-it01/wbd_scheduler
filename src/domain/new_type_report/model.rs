use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// The ultimate Output on success
pub struct NtrSummary {
    pub items_found: usize,
    pub items_inserted: usize,
}
pub enum NtrResult {
    Skipped { reason: String },
    Completed(NtrSummary),
}

// intermediate
#[derive(Debug, PartialEq, Serialize, Deserialize, FromRow)]
pub struct NtrData {
    pub head: NtrHead,
    pub number: Option<i8>,
    pub result: Decimal,
}
// Input
#[derive(Debug, PartialEq, Serialize, Deserialize, FromRow)]
pub struct NtrHead {
    pub item: String,
    pub wo_id: String,
    pub comp: String,
    pub msr_type: String,
    pub ppr_type: String,
    pub created_by: String,
}

// Refference
#[derive(Debug, PartialEq, Serialize, Deserialize, FromRow)]
pub struct NtrReff {
    pub created_by: String,
}

/*
$parent_msr_type = [
    'FO After Break in',
    'FO after sweeper',
    'FO Before Break in',
    'FO before sweeper',
    'FO DAAS',
    'Remark',
    'Attach',
];
$parent_msr_type_vc = [
    'Lebar Gulungan VC',
    'RDC',
    'Remark',
    'Attach',
];
$parent_msr_type_2 = [
    'Remark',
    'Attach',
];
$cone_msr = [
    'FO Cone',
    'Weight',
];
$sd_msr = [
    'Flexy',
];
$do_msr = [
    'OD',
    'Weight',
    'Height',
];
$dc_msr = [
    'Weight',
];
$cs_msr = [
    'Distance SD with CO',
    'Flatness CO',
    'Flatness SD',
    'PCD',
];
$yo_msr = [
    'Hole Center Yoke',
];
$tp_msr = [
    'ID',
];
$ma_msr = [
    'ID',
    'Weight',
];

*/
