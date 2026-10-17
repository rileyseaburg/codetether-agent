//! Mutable state for one legacy analysis invocation.
use super::*;
pub(super) struct AnalysisRun {
    pub(super) start: std::time::Instant,
    pub(super) iterations: usize,
    pub(super) total_input_tokens: usize,
    pub(super) total_output_tokens: usize,
    pub(super) tools: Vec<codetether_provider_core::ToolDefinition>,
    pub(super) messages: Vec<Message>,
    pub(super) llm_timeout_secs: u64,
    pub(super) requires_pattern_evidence: bool,
    pub(super) final_answer: Option<String>,
}
