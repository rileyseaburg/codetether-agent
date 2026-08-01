//! Stateful assembly of one OpenRouter streaming tool call.

use crate::provider::StreamChunk;

#[derive(Default)]
pub(super) struct ToolState {
    id: Option<String>,
    name: Option<String>,
    pending_arguments: String,
    started: bool,
}

impl ToolState {
    pub(super) fn apply(&mut self, delta: &super::types::ToolCall, output: &mut Vec<StreamChunk>) {
        if let Some(id) = delta.id.as_deref().filter(|id| !id.is_empty()) {
            self.id = Some(id.to_string());
        }
        if let Some(function) = &delta.function {
            if let Some(name) = function.name.as_deref().filter(|value| !value.is_empty()) {
                self.name = Some(name.to_string());
            }
            if let Some(arguments) = function.arguments.as_deref().filter(|v| !v.is_empty()) {
                self.pending_arguments.push_str(arguments);
            }
        }
        self.start_and_flush(output);
    }

    fn start_and_flush(&mut self, output: &mut Vec<StreamChunk>) {
        let (Some(id), Some(name)) = (self.id.clone(), self.name.clone()) else {
            return;
        };
        if !self.started {
            output.push(StreamChunk::ToolCallStart {
                id: id.clone(),
                name,
            });
            self.started = true;
        }
        if !self.pending_arguments.is_empty() {
            output.push(StreamChunk::ToolCallDelta {
                id,
                arguments_delta: std::mem::take(&mut self.pending_arguments),
            });
        }
    }

    pub(super) fn finish(&self, output: &mut Vec<StreamChunk>) {
        if self.started
            && let Some(id) = &self.id
        {
            output.push(StreamChunk::ToolCallEnd { id: id.clone() });
        }
    }
}
