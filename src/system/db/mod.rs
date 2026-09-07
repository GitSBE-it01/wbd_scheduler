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

pub trait DatabaseCacheable: Cacheable + Sized {
    async fn fetch_db(state: &AppState) -> Result<Vec<Self>, DbError>;
}
