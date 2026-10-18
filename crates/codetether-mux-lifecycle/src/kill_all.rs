//! Concurrent shutdown of every registered mux server.

use anyhow::{Result, bail};
use futures::future::join_all;

use codetether_mux_registry::MuxRecord;

/// Shut down every registered mux server concurrently.
///
/// # Arguments
/// None.
/// # Returns
/// Unit once every server shutdown attempt succeeds.
/// # Errors
/// Returns discovery errors or a summary of failed shutdown attempts.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// codetether_mux_lifecycle::kill_all::run().await?;
/// # Ok(()) }
/// ```
pub async fn run() -> Result<()> {
    let records = codetether_mux_registry::list().await?;
    run_records(records).await
}

/// Shut down a supplied collection of mux servers concurrently.
///
/// # Arguments
/// records identifies the server processes to shut down.
/// # Returns
/// Unit once every server shutdown attempt succeeds.
/// # Errors
/// Returns a summary of failed shutdown attempts.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// codetether_mux_lifecycle::kill_all::run_records(Vec::new()).await?;
/// # Ok(()) }
/// ```
pub async fn run_records(records: Vec<MuxRecord>) -> Result<()> {
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
