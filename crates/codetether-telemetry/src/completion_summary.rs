//! Latest reported completion model, duration and token counts.

/// Values reported for the most recently completed model request.
///
/// Each value remains optional because providers can omit usage or timing
/// information. Resetting a frontend clears the summary to its default.
///
/// # Examples
/// ```
/// let mut summary = codetether_telemetry::CompletionSummary::default();
/// assert!(summary.model.is_none());
/// assert!(summary.latency_ms.is_none());
/// summary.prompt_tokens = Some(120);
/// summary.output_tokens = Some(64);
/// assert_eq!(summary.output_tokens, Some(64));
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CompletionSummary {
    /// Model identifier reported for the completion.
    pub model: Option<String>,
    /// Reported completion duration in milliseconds.
    pub latency_ms: Option<u64>,
    /// Reported input token count.
    pub prompt_tokens: Option<usize>,
    /// Reported output token count.
    pub output_tokens: Option<usize>,
}
