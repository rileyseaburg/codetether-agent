//! Accumulated state for one transactional stream drain.

use super::finalize::ToolAccumulator;
use crate::provider::{ContentPart, Usage};
use std::collections::HashMap;

pub(super) struct DrainState {
    pub(super) text: String,
    pub(super) thinking: String,
    pub(super) reasoning_signature: Option<String>,
    pub(super) tools: Vec<ToolAccumulator>,
    pub(super) idx: HashMap<String, usize>,
    pub(super) usage: Usage,
    pub(super) completed: Vec<ContentPart>,
}

impl DrainState {
    pub(super) fn new() -> Self {
        Self {
            text: String::new(),
            thinking: String::new(),
            reasoning_signature: None,
            tools: Vec::new(),
            idx: HashMap::new(),
            usage: Usage::default(),
            completed: Vec::new(),
        }
    }

    pub(super) fn finish(self) -> (crate::provider::CompletionResponse, Vec<ContentPart>) {
        let response = super::finalize::build_response(
            self.thinking,
            self.reasoning_signature,
            self.text,
            self.tools,
            self.usage,
        );
        (response, self.completed)
    }
}
