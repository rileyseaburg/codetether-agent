//! Prepared trace, model tools, and initial conversation.

use crate::{
    context_trace::ContextTrace,
    traits::{LlmMessage, ToolDefinition},
};
use uuid::Uuid;

/// Opaque state carried between prepare and the iterative loop.
pub(in crate::router) struct Prepared {
    pub trace_id: Uuid,
    pub trace: ContextTrace,
    pub tools: Vec<ToolDefinition>,
    pub conversation: Vec<LlmMessage>,
    pub summary_mode: bool,
    pub input_tokens: usize,
}
