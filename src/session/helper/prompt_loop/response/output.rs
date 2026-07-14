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
        publish_thinking(runner, step, &thinking);
    }
    if !text.is_empty() {
        runner.progress.output.push_str(&format!("{text}\n"));
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

fn publish_thinking(runner: &Runner<'_>, step: usize, thinking: &str) {
    let Some(bus) = &runner.session.bus else {
        return;
    };
    bus.handle(&runner.session.agent).send_with_correlation(
        format!("agent.{}.thinking", runner.session.agent),
        crate::bus::BusMessage::AgentThinking {
            agent_id: runner.session.agent.clone(),
            thinking: super::super::super::live_bus::compact_thinking(thinking),
            step,
        },
        Some(runner.progress.turn_id.clone()),
    );
}
