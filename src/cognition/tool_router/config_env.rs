//! Environment-variable loading for [`ToolRouterConfig`](super::ToolRouterConfig).

use super::ToolRouterConfig;
use crate::cognition::CandleDevicePreference;

impl ToolRouterConfig {
    /// Build from environment variables.
    ///
    /// | Variable | Description |
    /// |----------|-------------|
    /// | `CODETETHER_TOOL_ROUTER_ENABLED` | `true` / `1` to activate |
    /// | `CODETETHER_TOOL_ROUTER_MODEL_PATH` | Path to `.gguf` model |
    /// | `CODETETHER_TOOL_ROUTER_TOKENIZER_PATH` | Path to `tokenizer.json` |
    /// | `CODETETHER_TOOL_ROUTER_ARCH` | Architecture hint (default: `gemma3`) |
    /// | `CODETETHER_TOOL_ROUTER_DEVICE` | `auto` / `cpu` / `cuda` |
    /// | `CODETETHER_TOOL_ROUTER_MAX_TOKENS` | Max decode tokens |
    /// | `CODETETHER_TOOL_ROUTER_TEMPERATURE` | Sampling temp (default: 0.1) |
    /// | `CODETETHER_FUNCTIONGEMMA_DISABLED` | Kill switch, defaults to `true` |
    pub fn from_env() -> Self {
        Self {
            enabled: flag("CODETETHER_TOOL_ROUTER_ENABLED", false) && !kill_switch(),
            model_path: var("CODETETHER_TOOL_ROUTER_MODEL_PATH"),
            tokenizer_path: var("CODETETHER_TOOL_ROUTER_TOKENIZER_PATH"),
            arch: var("CODETETHER_TOOL_ROUTER_ARCH").unwrap_or_else(|| "gemma3".to_string()),
            device: var("CODETETHER_TOOL_ROUTER_DEVICE")
                .map(|v| CandleDevicePreference::from_env(&v))
                .unwrap_or(CandleDevicePreference::Auto),
            max_tokens: parsed("CODETETHER_TOOL_ROUTER_MAX_TOKENS", 256),
            temperature: parsed("CODETETHER_TOOL_ROUTER_TEMPERATURE", 0.1),
        }
    }
}

/// Temporary safety default: keep FunctionGemma disabled unless explicitly
/// unblocked. This prevents local CPU/GPU contention in normal CLI/TUI runs.
fn kill_switch() -> bool {
    flag("CODETETHER_FUNCTIONGEMMA_DISABLED", true)
}

fn var(name: &str) -> Option<String> {
    std::env::var(name).ok()
}

fn flag(name: &str, default: bool) -> bool {
    var(name)
        .map(|v| matches!(v.as_str(), "1" | "true" | "yes"))
        .unwrap_or(default)
}

fn parsed<T: std::str::FromStr>(name: &str, default: T) -> T {
    var(name).and_then(|v| v.parse().ok()).unwrap_or(default)
}
