//! Single-entry constructor for Vertex AI Anthropic catalog rows.
//!
//! # Examples
//!
//! ```rust
//! use codetether_agent::provider::vertex_anthropic::vertex_anthropic_model::entry;
//!
//! let info = entry("claude-opus-5", "Claude Opus 5", 1_000_000, 128_000, 15.0, 75.0);
//! assert_eq!(info.name, "Claude Opus 5 (Vertex AI)");
//! assert_eq!(info.max_output_tokens, Some(128_000));
//! ```

use super::super::ModelInfo;

/// Build one Vertex AI Anthropic [`ModelInfo`] row.
///
/// # Arguments
///
/// * `id` — Vertex publisher model ID (for example `claude-opus-5`).
/// * `name` — Display name; ` (Vertex AI)` is appended.
/// * `context` — Context window in tokens.
/// * `max_output` — Maximum output tokens.
/// * `input_cost` — USD per million input tokens.
/// * `output_cost` — USD per million output tokens.
///
/// # Returns
///
/// A [`ModelInfo`] owned by `vertex-anthropic`; every Claude model on
/// Vertex supports vision, tools, and streaming.
pub fn entry(
    id: &str,
    name: &str,
    context: usize,
    max_output: usize,
    input_cost: f64,
    output_cost: f64,
) -> ModelInfo {
    ModelInfo {
        id: id.to_string(),
        name: format!("{name} (Vertex AI)"),
        provider: "vertex-anthropic".to_string(),
        context_window: context,
        max_output_tokens: Some(max_output),
        supports_vision: true,
        supports_tools: true,
        supports_streaming: true,
        input_cost_per_million: Some(input_cost),
        output_cost_per_million: Some(output_cost),
    }
}
