//! Unsupported-architecture error reporting for Candle loading.

use anyhow::anyhow;

/// Gemma architecture identifiers, gated behind the `functiongemma` feature.
pub(super) const GEMMA: &[&str] = &["gemma", "gemma2", "gemma3", "gemma-embedding"];

/// Build the error for an architecture this build cannot load.
pub(super) fn unsupported(other: &str) -> anyhow::Error {
    if !cfg!(feature = "functiongemma") && GEMMA.contains(&other) {
        return anyhow!(
            "gemma architecture '{other}' requires the 'functiongemma' feature; rebuild with --features functiongemma"
        );
    }
    let extra = if cfg!(feature = "functiongemma") {
        ", gemma/gemma2/gemma3"
    } else {
        ""
    };
    anyhow!(
        "unsupported candle architecture '{other}' (supported: llama, qwen2, qwen3, qwen3_moe{extra})"
    )
}
