//! Terminal OpenRouter stream handling.

use super::parser::Parser;
use crate::provider::{StreamChunk, Usage};

impl Parser {
    pub(super) fn finish(
        &mut self,
        usage: Option<super::super::OpenRouterUsage>,
        output: &mut Vec<StreamChunk>,
    ) {
        if self.done {
            return;
        }
        for tool in self.tools.values() {
            tool.finish(output);
        }
        let usage = usage.map(|u| Usage {
            prompt_tokens: u.prompt_tokens,
            completion_tokens: u.completion_tokens,
            total_tokens: u.total_tokens,
            ..Default::default()
        });
        output.push(StreamChunk::Done { usage });
        self.done = true;
    }
}
