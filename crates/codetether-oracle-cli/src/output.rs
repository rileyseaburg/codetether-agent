//! Render validation results and persistence outcomes.
use crate::{json::oracle_json, status::oracle_status_line};
use anyhow::Result;
use codetether_rlm::oracle::{OracleResult, OracleTracePersistResult};
use serde_json::json;

pub(super) fn print(
    json: bool,
    oracle_result: &OracleResult,
    persist: Option<OracleTracePersistResult>,
) -> Result<()> {
    if json {
        let out = json!({
            "oracle": oracle_json(&oracle_result),
            "record": oracle_result.to_record(),
            "persist": persist,
        });
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else {
        println!("{}", oracle_status_line(&oracle_result));
        if let Some(ref p) = persist {
            if p.uploaded {
                println!(
                    "[persist: uploaded ✓] {}",
                    p.remote_url.clone().unwrap_or_default()
                );
            } else {
                println!("[persist: spooled —] {}", p.spooled_path);
            }
            if let Some(ref w) = p.warning {
                println!("warning: {}", w);
            }
        }
    }

    Ok(())
}
