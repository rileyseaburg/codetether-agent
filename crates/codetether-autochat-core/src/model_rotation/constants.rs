//! Provider discovery and resolution precedence.

pub(super) const PRIORITY_PROVIDERS: [&str; 8] = [
    "openai-codex",
    "minimax",
    "zai",
    "github-copilot",
    "github-copilot-enterprise",
    "openrouter",
    "minimax-credits",
    "openai",
];

pub(super) const RESOLUTION_FALLBACKS: [&str; 12] = [
    "openai-codex",
    "minimax",
    "zai",
    "github-copilot",
    "github-copilot-enterprise",
    "openrouter",
    "minimax-credits",
    "openai",
    "anthropic",
    "novita",
    "moonshotai",
    "google",
];
