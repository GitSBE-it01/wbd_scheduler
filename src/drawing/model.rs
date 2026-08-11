use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, PartialEq, Serialize, Deserialize, FromRow)]
pub struct ItemData {
    pub item: String,
    pub item_desc: Option<String>,
    pub drawing_path: String,
}
