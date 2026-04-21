//! Input/output token counts with field names that match most provider APIs.

use serde::{Deserialize, Serialize};

/// Token counts using the `input_tokens` / `output_tokens` naming convention
/// favoured by most provider wire formats (Anthropic, Bedrock, OpenAI v2).
///
/// Prefer [`super::TokenTotals`] for internal aggregation; use this type when
/// you're crossing a serialization boundary that expects those field names.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::telemetry::TokenCounts;
///
/// let c = TokenCounts::new(2_048, 512);
/// assert_eq!(c.input_tokens, 2_048);
/// assert_eq!(c.output_tokens, 512);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenCounts {
    /// Prompt / input tokens billed at the input rate.
    pub input_tokens: u64,
    /// Completion / output tokens billed at the output rate.
    pub output_tokens: u64,
}

impl TokenCounts {
    /// Construct a new pair.
    pub fn new(input_tokens: u64, output_tokens: u64) -> Self {
        Self {
            input_tokens,
            output_tokens,
        }
    }
}
