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
/// use codetether_agent::provider::pricing::pricing_for_model;
///
/// let (input, output) = pricing_for_model("claude-opus-4-7");
/// assert!(input > 0.0 && output > 0.0);
/// ```
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

/// Multiplier applied to `input_price` for cache-read tokens. Varies by
/// provider family:
///
/// * Anthropic / Bedrock Claude — 10% (0.1×)
/// * OpenAI (GPT-4o, GPT-5, Codex) — 50% (0.5×)
/// * Google Gemini — 25% (0.25×)
/// * Z.AI GLM / Moonshot — 20% (0.2×) *(best-effort default)*
///
/// Returns `0.1` as a conservative default so pricing never silently
/// over-counts for an unknown model.
pub fn cache_read_multiplier(model: &str) -> f64 {
    let m = model.to_ascii_lowercase();
    if m.contains("claude") || m.contains("anthropic") {
        0.10
    } else if m.contains("gpt-") || m.contains("codex") || m.contains("o1") || m.contains("o3") {
        0.50
    } else if m.contains("gemini") {
        0.25
    } else if m.contains("glm") || m.contains("kimi") || m.contains("k1.") {
        0.20
    } else {
        0.10
    }
}

/// Multiplier applied to `input_price` for cache-write tokens. Only
/// Anthropic/Bedrock bill a surcharge for cache writes (1.25×). For
/// providers that cache implicitly (OpenAI, Gemini, GLM) this is `0.0`
/// because the write is bundled into the regular input price.
pub fn cache_write_multiplier(model: &str) -> f64 {
    let m = model.to_ascii_lowercase();
    if m.contains("claude") || m.contains("anthropic") {
        1.25
    } else {
        0.0
    }
}

/// Estimate the running cost (USD) of the current session, using the
/// global [`crate::telemetry::TOKEN_USAGE`] counters and
/// [`pricing_for_model`]. Applies provider-specific prompt-cache
/// multipliers via [`cache_read_multiplier`] and [`cache_write_multiplier`].
///
/// Assumes `prompt_tokens` in the counter already excludes cached input —
/// which is how [`crate::telemetry::AtomicTokenCounter::record_model_usage_with_cache`]
/// is invoked from every provider adapter.
pub fn session_cost_usd() -> f64 {
    use crate::telemetry::TOKEN_USAGE;

    TOKEN_USAGE
        .model_snapshots()
        .iter()
        .map(|s| {
            let (input_price, output_price) = pricing_for_model(&s.name);
            let (cache_read, cache_write) = TOKEN_USAGE.cache_usage_for(&s.name);
            let per_million = |n: u64, price: f64| (n as f64 / 1_000_000.0) * price;
            per_million(s.prompt_tokens, input_price)
                + per_million(s.completion_tokens, output_price)
                + per_million(cache_read, input_price * cache_read_multiplier(&s.name))
                + per_million(cache_write, input_price * cache_write_multiplier(&s.name))
        })
        .sum()
}
