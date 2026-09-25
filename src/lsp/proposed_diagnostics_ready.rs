//! Wait for a cold real rust-analyzer to analyze the proposed test document.

use crate::lsp::{DiagnosticInfo, LspActionResult, client::LspClient, path_to_uri};
use std::path::Path;
use tokio::time::{Duration, sleep, timeout};

pub(super) async fn broken_content(client: &LspClient, file: &Path) -> LspActionResult {
    // Cold workspace loading uses the server's configured startup budget.
    timeout(Duration::from_millis(client.config.timeout_ms), async {
        client
            .diagnostics_for_content(file, "pub fn broken( {\n")
            .await
            .expect("rust-analyzer diagnostics request failed");
        let uri = path_to_uri(file);
        loop {
            let publications = client.transport.diagnostics_snapshot().await;
            if let Some(diagnostics) = publications.get(&uri).filter(|items| !items.is_empty()) {
                return LspActionResult::Diagnostics {
                    diagnostics: diagnostics
                        .iter()
                        .cloned()
                        .map(|value| DiagnosticInfo::from((uri.clone(), value)))
                        .collect(),
                };
            }
            // Do not keep changing the version and cancelling in-flight analysis.
            sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .expect("rust-analyzer returned no diagnostics after workspace warmup")
}
