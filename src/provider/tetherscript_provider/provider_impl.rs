//! [`Provider`] trait impl — delegates every method to tetherscript.
//!
//! `LoadedPlugin` is `!Send`, so we run hooks inside `spawn_blocking`.

use super::runner::TetherScriptProvider;
use super::super::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason,
    Message, ModelInfo, Provider, Role, StreamChunk, Usage,
};
use anyhow::Result;
use async_trait::async_trait;
use futures::stream::BoxStream;

#[async_trait]
impl Provider for TetherScriptProvider {
    fn name(&self) -> &str { self.name_str() }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.call_list_models())
            .await?
    }

    async fn complete(&self, req: CompletionRequest) -> Result<CompletionResponse> {
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
            let text = r["content"].as_str().unwrap_or("").to_string();
            Ok(CompletionResponse {
                message: Message {
                    role: Role::Assistant,
                    content: vec![ContentPart::Text { text }],
                },
                usage: Usage::default(),
                finish_reason: FinishReason::Stop,
            })
        }).await?
    }

    async fn complete_stream(
        &self, _: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        anyhow::bail!("ts: no stream")
    }
}
