//! Streaming event and agent-bus publication for assistant output.

use super::super::Runner;
use crate::provider::{CompletionResponse, ContentPart};
use crate::session::SessionEvent;

/// Publishes assistant thinking/text and accumulates final output.
pub(super) async fn emit(runner: &mut Runner<'_>, step: usize, response: &CompletionResponse) {
    let thinking = collect(&response.message.content, true);
    let text = collect(&response.message.content, false);
    if let Some(tx) = &runner.events {
        if !thinking.is_empty() {
            let _ = tx
                .send(SessionEvent::ThinkingComplete(thinking.clone()))
                .await;
        }
        if !text.is_empty() {
            let _ = tx.send(SessionEvent::TextChunk(text.clone())).await;
            let _ = tx.send(SessionEvent::TextComplete(text.clone())).await;
        }
    }
    if !thinking.is_empty() {
        super::assistant_bus::thinking(runner, step, &thinking);
    }
    if !text.is_empty() {
        super::assistant_bus::text(runner, &text);
        // Text accompanying a tool call is a preamble, not the answer. Keeping
        // it produced answers made entirely of intent ("Now let me read...").
        if super::narration::is_preamble(&response.message.content) {
            tracing::debug!(step, "withholding pre-tool narration from final answer");
        } else {
            runner.progress.output.push_str(&format!("{text}\n"));
        }
    }
}

fn collect(parts: &[ContentPart], thinking: bool) -> String {
    parts
        .iter()
        .filter_map(|part| match (part, thinking) {
            (ContentPart::Thinking { text, .. }, true) => Some(text.as_str()),
            (ContentPart::Text { text }, false) => Some(text.as_str()),
            _ => None,
        })
        .filter(|text| !text.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}
