//! Compose evidence and memory prompt sections in their established order.
/// Complete text sections supplied to the evidence-prompt composer.
/// # Examples
/// ```
/// use codetether_evidence_prompt::assembly::{PromptSections, render};
/// let parts = PromptSections {
///     system_prompt: "base".into(), guardrails: String::new(),
///     failure_attribution: "", capabilities: String::new(), trapdoor: "",
///     memory_protocol: "", workflow_templates: String::new(), writeback: "",
///     core: String::new(), prefetch: String::new(), ids: String::new(),
/// };
/// assert!(render(parts).starts_with("base"));
/// ```
pub struct PromptSections<'a> {
    /// Base system prompt.
    pub system_prompt: String,
    /// Validation evidence instructions.
    pub guardrails: String,
    /// Rules for attributing failures.
    pub failure_attribution: &'a str,
    /// Runtime capability guidance.
    pub capabilities: String,
    /// Missing-context guidance.
    pub trapdoor: &'a str,
    /// Memory source and access rules.
    pub memory_protocol: &'a str,
    /// Task-specific proof requirements.
    pub workflow_templates: String,
    /// Durable-memory writeback guidance.
    pub writeback: &'a str,
    /// Caller-loaded core memory or its disabled notice.
    pub core: String,
    /// Caller-loaded prefetched facts or their disabled notice.
    pub prefetch: String,
    /// Runtime evidence classification note.
    pub ids: String,
}

/// Join prepared prompt sections in the established order and spacing.
/// # Arguments
/// `parts` contains rendered instructions and caller-supplied context text.
/// # Returns
/// A single prompt without performing memory, session or filesystem access.
/// # Examples
/// See [PromptSections] for a complete construction example.
pub fn render(parts: PromptSections<'_>) -> String {
    format!(
        "{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}{}",
        parts.system_prompt,
        parts.guardrails,
        parts.failure_attribution,
        parts.capabilities,
        parts.trapdoor,
        parts.memory_protocol,
        parts.workflow_templates,
        parts.writeback,
        parts.core,
        parts.prefetch,
        parts.ids
    )
}
