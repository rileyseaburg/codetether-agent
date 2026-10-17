//! Inputs specific to worker provider, approval, and output adapters.

use super::session_response::ResponseContext;
use crate::provider::{Provider, ToolDefinition};
use std::sync::Arc;

pub(super) struct Context<'a> {
    pub provider: Arc<dyn Provider>,
    pub model: &'a str,
    pub system_prompt: &'a str,
    pub tool_definitions: &'a [ToolDefinition],
    pub response: ResponseContext<'a>,
}
