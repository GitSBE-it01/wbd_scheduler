pub mod error;
pub mod model;
pub mod process;
use crate::system::cache;
use crate::system::db::ebom_parent::EBomParent;
use crate::system::state::AppState;
use error::EBomError;
use model::{EBomJoin, EBomOutcome, EBomPrJoin, EBomResult, EBomSummary};
use process::*;

pub async fn run_ebom(state: &AppState) -> Result<EBomOutcome, EBomError> {
    let store = cache::CacheEngine::new("//192.168.2.103/xampp/htdocs/wbd/_cache/");
    let cached = (
        store.get::<EBomJoin>(state, &"join_all".to_string()),
        store.get::<EBomJoin>(state, &"join_active".to_string()),
        store.get::<EBomPrJoin>(state, &"join_pr_all".to_string()),
        store.get::<EBomPrJoin>(state, &"join_pr_active".to_string()),
    );
    match cached {
        (Some(_), Some(_), Some(_), Some(_)) => Ok(EBomOutcome::Skipped {
            reason: "Cache still ok".to_string(),
        }),
        _ => {
            let raw = store.db_handler(state, "ebom_base".to_string()).await?;
            let base_cnt = &raw.data.len();
            let prcs = records_scan(raw.data)?;
            let cnt_join_all = &prcs.join_all.len();
            let cnt_join_active = &prcs.join_active.len();
            let cnt_join_pr_all = &prcs.join_pr_all.len();
            let cnt_join_pr_active = &prcs.join_pr_active.len();
            store.put::<EBomJoin>(state, &"ebom_join_all".to_string(), prcs.join_all)?;
            store.put::<EBomJoin>(state, &"ebom_join_active".to_string(), prcs.join_active)?;
            store.put::<EBomPrJoin>(state, &"ebom_join_pr_all".to_string(), prcs.join_pr_all)?;
            store.put::<EBomPrJoin>(
                state,
                &"ebom_join_pr_active".to_string(),
                prcs.join_pr_active,
            )?;
            Ok(EBomOutcome::Completed(EBomSummary {
                join_all: *cnt_join_all,
                join_active: *cnt_join_active,
                join_pr_all: *cnt_join_pr_all,
                join_pr_active: *cnt_join_pr_active,
                base: *base_cnt,
            }))
        }
    }
}
