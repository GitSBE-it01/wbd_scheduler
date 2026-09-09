use crate::core::ebom_parent::error::EBomError;
use crate::core::ebom_parent::model::{EBomJoin, EBomPrJoin, EBomResult};
use crate::system::db::ebom_parent::EBomParent;
use std::fmt::Write;

pub fn records_scan(mut raw_data: Vec<EBomParent>) -> Result<EBomResult, EBomError> {
    raw_data.sort_by(|a, b| (&a.comp, &a.parent_item).cmp(&(&b.comp, &b.parent_item)));
    let mut join_all = Vec::with_capacity(raw_data.len() / 2);
    let mut join_active = Vec::with_capacity(raw_data.len() / 2);
    let mut join_pr_all = Vec::with_capacity(raw_data.len() / 2);
    let mut join_pr_active = Vec::with_capacity(raw_data.len() / 2);

    for chunk in raw_data.chunk_by(|a, b| a.comp == b.comp) {
        let comp = chunk[0].comp.clone();
        let comp_desc = chunk[0].comp_desc.clone();
        let comp_stat = chunk[0].comp_stat.clone();
        let comp2 = format!(
            "{} -- {} ({})",
            chunk[0].comp.clone(),
            chunk[0].comp_desc.clone(),
            chunk[0].comp_stat.clone(),
        );
        // 2. Format ALL parents for this component
        let parent_all = chunk
            .iter()
            .map(|dt| {
                format!(
                    "- {} -- {} ({})",
                    dt.parent_item, dt.parent_desc, dt.parent_stat
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        // 3. Format ACTIVE parents (excluding SPSD and OBSOLETE)
        let parent_active = chunk
            .iter()
            .filter(|dt| dt.parent_stat != "SPSD" && dt.parent_stat != "OBSOLETE")
            .map(|dt| {
                format!(
                    "- {} -- {} ({})",
                    dt.parent_item, dt.parent_desc, dt.parent_stat
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        join_all.push(EBomJoin {
            comp: comp2.clone(),
            parent: parent_all.clone(),
        });

        if !parent_active.is_empty() {
            join_active.push(EBomJoin {
                comp: comp2,
                parent: parent_active.clone(),
            });
        }
        join_pr_all.push(EBomPrJoin {
            comp: comp.clone(),
            comp_desc: comp_desc.clone(),
            comp_stat: comp_stat.clone(),
            parent: parent_all,
        });

        if !parent_active.is_empty() {
            join_pr_active.push(EBomPrJoin {
                comp,
                comp_desc,
                comp_stat,
                parent: parent_active,
            });
        }
    }

    Ok(EBomResult {
        join_all,
        join_active,
        join_pr_all,
        join_pr_active,
    })
}
