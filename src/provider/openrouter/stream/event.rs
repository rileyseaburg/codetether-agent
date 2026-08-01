//! Decoding and dispatch of one OpenRouter SSE event.

use super::parser::Parser;
use super::types::Response;
use crate::provider::StreamChunk;

impl Parser {
    pub(super) fn parse_line(&mut self, line: &str, output: &mut Vec<StreamChunk>) {
        if line.is_empty() || self.done {
            return;
        }
        if line == "data: [DONE]" {
            self.finish(None, output);
            return;
        }
        let Some(data) = line.strip_prefix("data: ") else {
            return;
        };
        let Ok(response) = serde_json::from_str::<Response>(data) else {
            return;
        };
        let Some(choice) = response.choices.first() else {
            return;
        };
        if let Some(reasoning) = choice.delta.reasoning.as_deref().filter(|s| !s.is_empty()) {
            output.push(StreamChunk::Thinking(reasoning.to_string()));
        }
        if let Some(content) = choice.delta.content.as_deref().filter(|s| !s.is_empty()) {
            output.push(StreamChunk::Text(content.to_string()));
        }
        for call in choice.delta.tool_calls.iter().flatten() {
            self.tools
                .entry(call.index)
                .or_default()
                .apply(call, output);
        }
        if matches!(choice.finish_reason.as_deref(), Some("stop" | "tool_calls")) {
            self.finish(response.usage, output);
        }
    }
}
