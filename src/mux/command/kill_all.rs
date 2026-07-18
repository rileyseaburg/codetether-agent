//! Concurrent shutdown of every registered mux session.

use anyhow::{Result, bail};
use futures::future::join_all;

pub(super) async fn run() -> Result<()> {
    let records = crate::mux::registry::list().await?;
    run_records(records).await
}

pub(in crate::mux) async fn run_records(
    records: Vec<crate::mux::registry::MuxRecord>,
) -> Result<()> {
    if records.is_empty() {
        println!("no mux sessions");
        return Ok(());
    }
    let total = records.len();
    let attempts = records.into_iter().map(|record| async move {
        let name = record.name.clone();
        let result = super::kill::run_record(record).await;
        (name, result)
    });
    let failures: Vec<_> = join_all(attempts)
        .await
        .into_iter()
        .filter_map(|(name, result)| result.err().map(|error| (name, error)))
        .collect();
    if failures.is_empty() {
        println!("stopped all {total} mux sessions");
        return Ok(());
    }
    for (name, error) in &failures {
        tracing::error!(session = name, %error, "Failed to stop mux session");
    }
    bail!(
        "failed to stop {} of {total} mux sessions: {}",
        failures.len(),
        failure_names(&failures)
    )
}

fn failure_names(failures: &[(String, anyhow::Error)]) -> String {
    failures
        .iter()
        .map(|(name, _)| name.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}
