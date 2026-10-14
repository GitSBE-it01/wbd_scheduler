pub mod dc_drawing;
pub use dc_drawing::DcDrawing;
pub mod ebom_parent;
pub use ebom_parent::EBomParent;
pub mod error;
pub mod item_dev;
pub mod pick_now;
pub use pick_now::{
    pn_demand::PnDemand, pn_dept_inv::PnInv, pn_past_id::PnPastID, pn_pic::PnPic,
    pn_result::PnResult,
};
pub mod qad;
use crate::system::cache::Cacheable;
use crate::system::state::AppState;
pub use error::DbError;
pub use qad::item::QadItemMstr;
use std::process::Output;

pub trait DatabaseCacheable: Cacheable + Sized {
    fn fetch_db(
        state: &AppState,
    ) -> impl std::future::Future<Output = Result<Vec<Self>, DbError>> + Send;
}
