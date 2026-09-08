//! Pull TypeScript diagnostics explicitly: repeated clean files need not publish again.

use super::{LspActionResult, client::LspClient};
use anyhow::Result;
#[path = "typescript_diagnostic.rs"]
mod diagnostic;
#[path = "typescript_request.rs"]
mod request;

const COMMAND: &str = "typescript.tsserverRequest";

pub(super) async fn request(client: &LspClient, uri: &str) -> Result<Option<LspActionResult>> {
    let supported = client
        .server_capabilities
        .read()
        .await
        .as_ref()
        .and_then(|caps| caps.execute_command_provider.as_ref())
        .is_some_and(|provider| provider.commands.iter().any(|name| name == COMMAND));
    if !supported {
        return Ok(None);
    }
    let mut diagnostics = Vec::new();
    for method in [
        "syntacticDiagnosticsSync",
        "semanticDiagnosticsSync",
        "suggestionDiagnosticsSync",
    ] {
        let items = request::run(client, uri, method).await?;
        diagnostics.extend(items.into_iter().map(|item| item.into_lsp(uri)));
    }
    Ok(Some(LspActionResult::Diagnostics { diagnostics }))
}
