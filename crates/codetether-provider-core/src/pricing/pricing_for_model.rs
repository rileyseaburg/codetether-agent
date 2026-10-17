//! Shared model pricing policy.
//! Canonical per-million-token pricing for known LLM models.
//!
//! This is the **single source of truth** for model pricing. Every
//! subsystem (TUI cost badge, cost guardrail, benchmark runner) must
//! delegate here rather than maintaining its own table.
//!
//! Prices are best-effort retail USD per 1M tokens `(input, output)`.
//! Unknown models fall back to a conservative default.

/// Return `(input_price_per_million, output_price_per_million)` in USD
/// for a given model identifier. Matching is case-insensitive substring.
///
/// # Examples
///
/// ```rust
/// use codetether_provider_core::pricing::pricing_for_model;
///
/// let (input, output) = pricing_for_model("claude-opus-4-7");
/// assert!(input > 0.0 && output > 0.0);
/// ```
///
/// # Arguments
/// * `model` - Model identifier matched using the existing lowercase rules.
/// # Returns
/// The stored input/output price pair per million tokens.
pub fn pricing_for_model(model: &str) -> (f64, f64) {
    let m = model.to_ascii_lowercase();

    // ── Most specific patterns first ───────────────────────────────
    if m.contains("gpt-4o-mini") {
        (0.15, 0.60)
    } else if m.contains("gpt-4o") {
        (2.50, 10.00)
    } else if m.contains("gpt-4-turbo") {
        (10.00, 30.00)
    } else if m.contains("gpt-5") {
        (5.00, 15.00)
    } else if m.contains("gpt-4") {
        (30.00, 60.00)
    } else if m.contains("claude-3-5-sonnet") || m.contains("claude-sonnet-4") {
        (3.00, 15.00)
    } else if m.contains("claude-3-5-haiku") || m.contains("claude-haiku") {
        (0.80, 4.00)
    } else if m.contains("claude-opus") {
        // Anthropic Opus 4.x retail: $15 in / $75 out per million.
        (15.00, 75.00)
    } else if m.contains("gemini-2.5-pro") || m.contains("gemini-2-pro") {
        (1.25, 10.00)
    } else if m.contains("gemini-1.5-pro") {
        (1.25, 5.00)
    } else if m.contains("gemini") {
        (0.075, 0.30)
    } else if m.contains("glm-5") || m.contains("glm-4") {
        (0.50, 1.50)
    } else if m.contains("kimi") || m.contains("k1.6") {
        (6.00, 6.00)
    } else if m.contains("k1.5") {
        (8.00, 8.00)
    } else {
        // Conservative default for unknown models.
        (1.00, 3.00)
    }
}
