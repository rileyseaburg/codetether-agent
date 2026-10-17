//! Static policy allowlists for spawned-agent model checks.
//!
//! This module isolates constant policy data from eligibility logic so
//! the runtime checks remain small and focused.
//!
//! # Examples
//!
//! ```ignore
//! assert!(SUBSCRIPTION_PROVIDERS.contains(&"openai-codex"));
//! ```

pub(super) const SUBSCRIPTION_PROVIDERS: &[&str] = &[
    "openai-codex",
    "github-copilot",
    "github-copilot-enterprise",
    "gemini-web",
    "local_cuda",
    "zai",
    "glm-5",
];

pub(super) const OPENROUTER_BUDGET_ALLOWLIST: &[&str] = &["qwen/qwen3.5-35ba3b"];
