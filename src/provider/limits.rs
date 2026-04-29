//! Canonical context-window limits for known LLM models.
//!
//! This is the **single source of truth** for
//! [`context_window_for_model`]. Every subsystem — session compaction,
//! RLM routing, TUI token badges, Bedrock estimates — must delegate here
//! rather than maintaining its own heuristic map.
//!
//! # Adding a new model
//!
//! Add a `contains` match arm below. Order matters: more specific
//! patterns must come before broader ones (e.g. `"claude-opus-4-7"`
//! before `"claude"`).
//!
//! # Examples
//!
//! ```rust
//! use codetether_agent::provider::limits::context_window_for_model;
//!
//! assert_eq!(context_window_for_model("zai/glm-5"), 200_000);
//! assert_eq!(context_window_for_model("kimi-k2.5"), 256_000);
//! assert_eq!(context_window_for_model("claude-opus-4-7"), 1_000_000);
//! assert_eq!(context_window_for_model("unknown-model"), 128_000);
//! ```

/// Return the context window size (in tokens) for known models.
///
/// Uses case-insensitive substring matching against the model identifier.
/// Returns 128 000 for unknown models (conservative for most modern LLMs).
pub fn context_window_for_model(model: &str) -> usize {
    let m = model.to_ascii_lowercase();

    // ── Most specific patterns first ───────────────────────────────
    if m.contains("claude-opus-4-7") || m.contains("claude-opus-4.7") || m.contains("4.7-opus") {
        1_000_000
    } else if m.contains("glm-5") || m.contains("glm5") {
        200_000
    } else if m.contains("kimi-k2") || m.contains("kimi.k2") {
        256_000
    } else if m.contains("gpt-5") {
        256_000
    } else if m.contains("gpt-4o") || m.contains("gpt-4-turbo") || m.contains("gpt-4") {
        128_000
    } else if m.contains("claude") {
        200_000
    } else if m.contains("gemini-2.5-pro") || m.contains("gemini-2-pro") {
        2_000_000
    } else if m.contains("gemini") {
        1_000_000
    } else if m.contains("minimax") || m.contains("m2.5") {
        256_000
    } else if m.contains("qwen") || m.contains("qwq") {
        131_072
    } else if m.contains("deepseek-v4") {
        1_048_576
    } else if m.contains("deepseek-r1") || m.contains("deepseek-v3") || m.contains("deepseek-chat") || m.contains("deepseek-reasoner") {
        128_000
    } else if m.contains("llama-4") || m.contains("llama4") {
        256_000
    } else if m.contains("nova-pro") || m.contains("nova-lite") || m.contains("nova-premier") {
        300_000
    } else if m.contains("nova-micro") || m.contains("nova-2") {
        128_000
    } else if m.contains("moonshot") || m.contains("k1.5") || m.contains("k1.6") {
        200_000
    } else if m.contains("mistral-large") || m.contains("magistral") {
        128_000
    } else if m.contains("mistral") {
        32_000
    } else if m.contains("jamba") {
        256_000
    } else {
        128_000 // conservative default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_models() {
        assert_eq!(context_window_for_model("zai/glm-5"), 200_000);
        assert_eq!(context_window_for_model("glm-5-pro"), 200_000);
        assert_eq!(context_window_for_model("kimi-k2.5"), 256_000);
        assert_eq!(
            context_window_for_model("anthropic.claude-opus-4-7-v1:0"),
            1_000_000
        );
        assert_eq!(context_window_for_model("claude-3-5-sonnet"), 200_000);
        assert_eq!(context_window_for_model("gpt-4o"), 128_000);
        assert_eq!(context_window_for_model("gpt-4o-mini"), 128_000);
        assert_eq!(context_window_for_model("gemini-2.0-flash"), 1_000_000);
        assert_eq!(context_window_for_model("gemini-1.5-pro"), 1_000_000);
        assert_eq!(context_window_for_model("minimax-m2.5"), 256_000);
        assert_eq!(context_window_for_model("qwen-2.5-coder"), 131_072);
    }

    #[test]
    fn case_insensitive() {
        assert_eq!(
            context_window_for_model("Claude-Opus-4-7"),
            context_window_for_model("claude-opus-4-7")
        );
    }

    #[test]
    fn unknown_falls_back() {
        assert_eq!(context_window_for_model("totally-unknown-model"), 128_000);
    }
}
