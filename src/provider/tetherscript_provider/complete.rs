//! Non-streaming completion via tetherscript `complete` hook.

use anyhow::{Result, anyhow};

use super::runner::TetherScriptProvider;
use crate::provider::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, Role, Usage,
};

impl TetherScriptProvider {
    /// Run the tetherscript `complete` hook and build a response.
    pub(crate) async fn complete_non_streaming(
        &self,
        req: CompletionRequest,
    ) -> Result<CompletionResponse> {
        let this = self.clone();
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
            let text = extract_text(&r)?;
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
}

/// Extract assistant text from either a normalized `{content}` map or a raw
/// OpenAI-style `{choices[0].message.content}` chat-completions response.
fn extract_text(response: &serde_json::Value) -> Result<String> {
    let response = response.get("ok").unwrap_or(response);
    if let Some(text) = response.get("content").and_then(|v| v.as_str()) {
        return Ok(text.to_string());
    }
    if let Some(text) = response
        .get("choices")
        .and_then(|c| c.as_array())
        .and_then(|c| c.first())
        .and_then(|c| c.get("message"))
        .and_then(|m| m.get("content"))
        .and_then(|c| c.as_str())
    {
        return Ok(text.to_string());
    }
    Err(anyhow!(
        "tetherscript provider response did not include assistant text content: {response}"
    ))
}
