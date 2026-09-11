//! Concurrent shutdown of every registered mux server.

use anyhow::{Result, bail};
use futures::future::join_all;

use crate::mux::registry::MuxRecord;

pub(super) async fn run() -> Result<()> {
    let records = crate::mux::registry::list().await?;
    run_records(records).await
}

pub(in crate::mux) async fn run_records(records: Vec<MuxRecord>) -> Result<()> {
    if records.is_empty() {
        println!("no mux sessions");
        return Ok(());
    }
    let total = records.len();
    let attempts = records.into_iter().map(|record| async move {
        let key = record.key.clone();
        (key, super::kill_server::shutdown(record).await)
    });
    let failures: Vec<_> = join_all(attempts)
        .await
        .into_iter()
        .filter_map(|(key, result)| result.err().map(|error| (key, error)))
        .collect();
    if failures.is_empty() {
        println!("stopped all {total} mux servers");
        return Ok(());
    }
    for (key, error) in &failures {
        tracing::error!(server = key, %error, "Failed to stop mux server");
    }
    let names: Vec<_> = failures.iter().map(|(key, _)| key.as_str()).collect();
    bail!(
        "failed to stop {} of {total} mux servers: {}",
        failures.len(),
        names.join(", ")
    )
}
