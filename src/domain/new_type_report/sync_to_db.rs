use crate::{
    core::state::AppState,
    domain::new_type_report::{error::NtrError, model::NtrData},
};
// use sqlx::{MySql, QueryBuilder};
use tracing::{error, info};

pub async fn insert_into_database(
    state: &AppState,
    records: Vec<NtrData>,
) -> Result<usize, NtrError> {
    if records.is_empty() {
        error!("!!>> No Data To Entry...");
        return Ok(0);
    }
    todo!()
}
