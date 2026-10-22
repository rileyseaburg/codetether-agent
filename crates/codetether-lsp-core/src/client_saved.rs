//! Save notifications for disk diagnostics, never for speculative content.
use crate::LspClient;
use anyhow::Result;
use lsp_types::{ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncSaveOptions};
use serde_json::{Value, json};

fn params(caps: Option<&ServerCapabilities>, uri: &str, content: &str) -> Option<Value> {
    let TextDocumentSyncCapability::Options(sync) = caps?.text_document_sync.as_ref()? else {
        return None;
    };
    let include_text = match sync.save.as_ref()? {
        TextDocumentSyncSaveOptions::Supported(false) => return None,
        TextDocumentSyncSaveOptions::Supported(true) => false,
        TextDocumentSyncSaveOptions::SaveOptions(options) => options.include_text.unwrap_or(false),
    };
    let mut value = json!({"textDocument": {"uri": uri}});
    if include_text {
        value["text"] = json!(content);
    }
    Some(value)
}

pub(crate) async fn notify(client: &LspClient, uri: &str, content: &str) -> Result<()> {
    let value = params(
        client.server_capabilities.read().await.as_ref(),
        uri,
        content,
    );
    if let Some(value) = value {
        client
            .transport
            .notify("textDocument/didSave", Some(value))
            .await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn saved_diagnostics_respect_server_capabilities() {
        for (save, expected) in [
            (json!(false), None),
            (json!(true), Some(false)),
            (json!({"includeText":true}), Some(true)),
        ] {
            let caps: ServerCapabilities =
                serde_json::from_value(json!({"textDocumentSync":{"save":save}})).unwrap();
            assert_eq!(
                params(Some(&caps), "file:///fixture.rs", "saved text")
                    .map(|p| p.get("text").is_some()),
                expected
            );
        }
        assert!(params(None, "file:///fixture.rs", "text").is_none());
    }
}
