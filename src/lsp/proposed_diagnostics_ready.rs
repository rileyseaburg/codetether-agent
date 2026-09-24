//! Wait for a cold real rust-analyzer to analyze the proposed test document.

use crate::lsp::{LspActionResult, client::LspClient};
use std::path::Path;
use tokio::time::{Duration, sleep, timeout};

pub(super) async fn broken_content(client: &LspClient, file: &Path) -> LspActionResult {
    timeout(Duration::from_secs(15), async {
        loop {
            let result = client
                .diagnostics_for_content(file, "pub fn broken( {\n")
                .await
                .expect("rust-analyzer diagnostics request failed");
            let LspActionResult::Diagnostics { diagnostics } = &result else {
                panic!("wrong diagnostic response")
            };
            if !diagnostics.is_empty() {
                return result;
            }
            // Initialization can publish an empty list before workspace loading.
            sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .expect("rust-analyzer returned no diagnostics after workspace warmup")
}
