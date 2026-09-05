//! Await an explicit publication for the requested document, not global activity.

use std::{collections::HashMap, future::Future, time::Duration};

use anyhow::{Context, Result};
use lsp_types::Diagnostic;

/// Wait for an explicit publication, including a valid empty diagnostic list.
///
/// # Arguments
/// `snapshot` reads current publications; `uri` selects the document; `timeout`
/// is the configured language-server deadline, including cold-start work.
///
/// # Returns
/// The diagnostics published for the requested document.
/// # Errors
/// Returns an error when no publication for `uri` arrives before the deadline.
pub(super) async fn wait<F, Fut>(
    mut snapshot: F,
    uri: &str,
    timeout: Duration,
) -> Result<Vec<Diagnostic>>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = HashMap<String, Vec<Diagnostic>>>,
{
    tokio::time::timeout(timeout, async {
        loop {
            // An explicitly published empty list is authoritative; absence is not.
            if let Some(diagnostics) = snapshot().await.remove(uri) {
                return diagnostics;
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    })
    .await
    .with_context(|| {
        format!("LSP diagnostics timeout for {uri}: no publication within {timeout:?}")
    })
}

#[cfg(test)]
#[path = "client_diagnostics_wait_tests.rs"]
mod tests;
