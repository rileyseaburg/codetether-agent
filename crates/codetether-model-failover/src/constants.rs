//! Defaults for cross-provider failover and local model selection.
//! Shared policy owns these values; environment overrides are resolved by helpers.

/// Default retry budget when no valid environment override is set.
pub const SMART_SWITCH_MAX_RETRIES: u8 = 3;

/// Cross-provider fallback order after same-provider candidates.
pub const SMART_SWITCH_PROVIDER_PRIORITY: [&str; 11] = [
    "minimax",
    "zai",
    "openai-codex",
    "openrouter",
    "github-copilot",
    "github-copilot-enterprise",
    "minimax-credits",
    "openai",
    "anthropic",
    "google",
    "gemini-web",
];

/// Default local inference model used when no override applies.
pub const AUTOCHAT_LOCAL_DEFAULT_MODEL: &str = "local_cuda/qwen3.5-9b";
