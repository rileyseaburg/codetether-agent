//! OpenAI-compatible usage wire contracts.
use serde::Serialize;

#[derive(Debug, Serialize)]
/// OpenAI-compatible OpenAiUsage wire representation.
/// # Examples
/// ```
/// let _type_name = std::any::type_name::<codetether_openai_wire::OpenAiUsage>();
/// ```
pub struct OpenAiUsage {
    /// prompt_tokens wire field.
    pub prompt_tokens: usize,
    /// completion_tokens wire field.
    pub completion_tokens: usize,
    /// total_tokens wire field.
    pub total_tokens: usize,
}
