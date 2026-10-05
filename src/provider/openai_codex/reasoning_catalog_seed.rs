//! Offline reasoning-effort defaults when no live catalog is available.
pub(super) fn levels(model: &str) -> &'static [&'static str] {
    match model {
        "gpt-6.1-sol" | "gpt-6-sol" | "gpt-6-astra" | "gpt-5.6-sol" | "gpt-5.6-terra" => {
            &["low", "medium", "high", "xhigh", "max", "ultra"]
        }
        "gpt-6-luna" | "gpt-reserve" | "gpt-5.6-luna" | "codex-auto-review" => {
            &["low", "medium", "high", "xhigh", "max"]
        }
        "gpt-5.5" | "gpt-5.5-fast" | "gpt-5.4" | "gpt-5.4-mini" | "gpt-5.3-codex-spark" => {
            &["low", "medium", "high", "xhigh"]
        }
        _ => &[],
    }
}
