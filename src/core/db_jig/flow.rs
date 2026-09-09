use crate::{
    core::state::AppState,
    // core::utilities::{read_cache::read_data, save_file::save_as_json, validation::is_cache_valid},
    domain::new_type_report::{
        error::NtrError,
        model::{NtrHead, NtrResult, NtrSummary, RawNtrHead},
        sync_to_db::{fetch_data, insert_into_database},
    },
};
use tracing::info;
pub async fn run(state: &AppState) -> Result<NtrResult, NtrError> {
    info!("->> Running NTR Sched");
    let records: Vec<RawNtrHead> = fetch_data(state).await?;
    let mut process_result: Vec<NtrHead> = Vec::with_capacity(records.len());
    let parent_msr_type = [
        "FO After Break in",
        "FO after sweeper",
        "FO Before Break in",
        "FO before sweeper",
        "FO DAAS",
        "Remark",
        "Attach",
    ];
    let parent_msr_type_vc = ["Lebar Gulungan VC", "RDC", "Remark", "Attach"];
    let parent_msr_type_2 = ["Remark", "Attach"];

    for rec in records {
        if rec.item.starts_with('1') {
            for msr_ty in parent_msr_type {
                if msr_ty == "Attach" || msr_ty == "Remark" {
                    process_result.push(NtrHead {
                        item: rec.item.clone(),
                        wo_id: rec.wo_id.clone(),
                        comp: rec.item.clone(),
                        msr_type: msr_ty.to_string(),
                        ppr_type: "remark".to_string(),
                    });
                } else {
                    process_result.push(NtrHead {
                        item: rec.item.clone(),
                        wo_id: rec.wo_id.clone(),
                        comp: rec.item.clone(),
                        msr_type: msr_ty.to_string(),
                        ppr_type: "product".to_string(),
                    });
                }
            }
        } else if rec.item.starts_with("2VC") || rec.item.starts_with("2vc") {
            for msr_ty in parent_msr_type_vc {
                if msr_ty == "Attach" || msr_ty == "Remark" {
                    process_result.push(NtrHead {
                        item: rec.item.clone(),
                        wo_id: rec.wo_id.clone(),
                        comp: rec.item.clone(),
                        msr_type: msr_ty.to_string(),
                        ppr_type: "remark".to_string(),
                    });
                } else {
                    process_result.push(NtrHead {
                        item: rec.item.clone(),
                        wo_id: rec.wo_id.clone(),
                        comp: rec.item.clone(),
                        msr_type: msr_ty.to_string(),
                        ppr_type: "product".to_string(),
                    });
                }
            }
        } else {
            for msr_ty in parent_msr_type_2 {
                process_result.push(NtrHead {
                    item: rec.item.clone(),
                    wo_id: rec.wo_id.clone(),
                    comp: rec.item.clone(),
                    msr_type: msr_ty.to_string(),
                    ppr_type: "remark".to_string(),
                });
            }
        }
        let comp_array: &[&str] = match &rec.comp {
            s if s.starts_with("3CO") => &["FO Cone", "Weight"],
            s if s.starts_with("3SD") => &["Flexy"],
            s if s.starts_with("3DO") => &["OD", "Weight", "Height"],
            s if s.starts_with("3DC") => &["Weight"],
            s if s.starts_with("3CS") => {
                &["Distance SD with CO", "Flatness CO", "Flatness SD", "PCD"]
            }
            s if s.starts_with("3YO") => &["Hole Center Yoke"],
            s if s.starts_with("3TP") => &["ID"],
            s if s.starts_with("3MA") => &["ID", "Weight"],
            _ => &[],
        };
        if !comp_array.is_empty() {
            for msr_ty in comp_array {
                process_result.push(NtrHead {
                    item: rec.item.clone(),
                    wo_id: rec.wo_id.clone(),
                    comp: rec.comp.clone(),
                    msr_type: msr_ty.to_string(),
                    ppr_type: "part".to_string(),
                });
            }
        }
    }
    for pc in process_result.iter().take(5) {
        println!(
            "check result for item: {}, wo_id: {}, comp: {}, and msr_type: {}",
            pc.item, pc.wo_id, pc.comp, pc.msr_type
        );
    }
    let items_found = process_result.len();
    info!("->> Start Inserting");
    let items_inserted = insert_into_database(state, process_result).await?;
    info!(count_inserted= ?items_inserted, "Inserting Result");
    // Step 3: Return the final Output
    Ok(NtrResult::Completed(NtrSummary {
        nt_hd_found: items_found, // Simplified for example
        nt_hd_inserted: items_inserted,
    }))
}
