use crate::system::cache::Cacheable;
use serde::{Deserialize, Serialize};

pub struct EBomSummary {
    pub base: usize,
    pub join_all: usize,
    pub join_active: usize,
    pub join_pr_all: usize,
    pub join_pr_active: usize,
}
pub enum EBomOutcome {
    Skipped { reason: String },
    Completed(EBomSummary),
}
pub struct EBomResult {
    pub join_all: Vec<EBomJoin>,
    pub join_active: Vec<EBomJoin>,
    pub join_pr_all: Vec<EBomPrJoin>,
    pub join_pr_active: Vec<EBomPrJoin>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EBomJoin {
    pub comp: String,
    pub parent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EBomPrJoin {
    pub comp: String,
    pub comp_desc: String,
    pub comp_stat: String,
    pub parent: String,
}

impl Cacheable for EBomPrJoin {
    type Key = String;

    fn namespace() -> &'static str {
        "ebom_parent" // Saved to ./cache_dir/scans/<key>.json
    }

    fn ttl() -> i64 {
        60 * 60 * 18 // Fresh for 1 hour
    }
}
impl Cacheable for EBomJoin {
    type Key = String;

    fn namespace() -> &'static str {
        "ebom_parent" // Saved to ./cache_dir/scans/<key>.json
    }

    fn ttl() -> i64 {
        60 * 60 * 18 // Fresh for 1 hour
    }
}
