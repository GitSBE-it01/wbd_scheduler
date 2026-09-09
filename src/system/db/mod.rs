pub mod dc_drawing;
pub mod ebom_parent;
pub mod error;
pub mod pn_demand;
pub mod pn_dept_inv;
pub mod pn_past_id;
pub mod pn_pic;
pub mod pn_result;
pub mod qad_item;
use crate::system::cache::Cacheable;
use crate::system::state::AppState;
use error::DbError;
use std::process::Output;

pub trait DatabaseCacheable: Cacheable + Sized {
    fn fetch_db(
        state: &AppState,
    ) -> impl std::future::Future<Output = Result<Vec<Self>, DbError>> + Send;
}
