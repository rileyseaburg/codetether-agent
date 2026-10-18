//! Oracle CLI sync responsibilities.
use crate::OracleSyncArgs;
use anyhow::Result;
use codetether_rlm::oracle::OracleTraceStorage;

pub(super) async fn execute_sync(args: OracleSyncArgs) -> Result<()> {
    let storage = OracleTraceStorage::from_env_or_vault().await;
    let stats = storage.sync_pending().await?;

    if args.json {
        println!("{}", serde_json::to_string_pretty(&stats)?);
    } else {
        println!(
            "synced={} retained={} failed={} pending_after={}",
            stats.uploaded, stats.retained, stats.failed, stats.pending_after
        );
    }

    Ok(())
}
