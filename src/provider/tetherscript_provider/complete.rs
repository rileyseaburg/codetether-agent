//! Non-streaming completion via tetherscript `complete` hook.

use anyhow::Result;

use super::runner::TetherScriptProvider;
use crate::provider::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason,
    Message, Role, Usage,
};

/// Run the tetherscript `complete` hook and build a response.
pub async fn complete(
    provider: &TetherScriptProvider,
    req: CompletionRequest,
) -> Result<CompletionResponse> {
    let this = provider.clone();
    let msgs = super::convert::messages(&req);
    let arg = serde_json::json!({
        "messages": msgs,
        "opts": {
            "model": req.model,
            "temperature": req.temperature.unwrap_or(0.7),
        }
    });
    tokio::task::spawn_blocking(move || {
        let r = this.call1_sync("complete", arg)?;
        let text = r["content"].as_str().unwrap_or("").to_string();
        Ok(CompletionResponse {
            message: Message {
                role: Role::Assistant,
                content: vec![ContentPart::Text { text }],
            },
            usage: Usage::default(),
            finish_reason: FinishReason::Stop,
        })
    })
    .await?
}
