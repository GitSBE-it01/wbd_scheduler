use crate::system::db::{pn_demand::PnDemand, pn_dept_inv::PnInv};

#[derive(Debug, Clone)]
pub enum Record {
    Demand(PnDemand),
    Inventory(PnInv),
}
