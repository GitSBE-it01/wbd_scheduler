// use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// The ultimate Output on success
pub struct NtrSummary {
    pub nt_hd_found: usize,
    pub nt_hd_inserted: usize,
}
// input and output
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct NtrHead {
    pub item: String,
    pub wo_id: String,
    pub comp: String,
    pub msr_type: String,
    pub ppr_type: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, FromRow)]
pub struct RawNtrHead {
    pub item: String,
    pub wo_id: String,
    pub comp: String,
}

// Input
