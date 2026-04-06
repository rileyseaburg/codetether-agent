//! TUI constants and configuration values

/// Sentinel value meaning "follow the latest message position" (top in newest-first chat view).
/// Kept as a legacy name to avoid touching many call sites.
pub const SCROLL_BOTTOM: usize = 1_000_000;

// Tool-call / tool-result rendering can carry very large JSON payloads (e.g. patches, file blobs).
// If we pretty-print + split/collect that payload on every frame, the TUI can appear to "stop
// rendering" after a few tool calls due to render-time CPU churn.
pub const TOOL_ARGS_PRETTY_JSON_MAX_BYTES: usize = 16_000;
pub const TOOL_ARGS_PREVIEW_MAX_LINES: usize = 10;
pub const TOOL_ARGS_PREVIEW_MAX_BYTES: usize = 6_000;
pub const TOOL_OUTPUT_PREVIEW_MAX_LINES: usize = 5;
pub const TOOL_OUTPUT_PREVIEW_MAX_BYTES: usize = 4_000;
pub const FILE_PICKER_MAX_ENTRIES: usize = 500;
pub const FILE_SHARE_MAX_BYTES: usize = 64 * 1024;
pub const FILE_PICKER_PREVIEW_MAX_BYTES: usize = 8 * 1024;
pub const FILE_PICKER_PREVIEW_MAX_LINES: usize = 14;
pub const FILE_PICKER_PREVIEW_DIR_ITEMS: usize = 10;
pub const FILE_PICKER_PAGE_STEP: usize = 12;
pub const MAIN_PROCESSING_WATCHDOG_TIMEOUT_SECS: u64 = 300;
pub const SMART_SWITCH_MAX_RETRIES: u8 = 3;
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
pub const AUTOCHAT_MAX_AGENTS: usize = crate::autochat::AUTOCHAT_MAX_AGENTS;
pub const AUTOCHAT_DEFAULT_AGENTS: usize = crate::autochat::AUTOCHAT_DEFAULT_AGENTS;
pub const AUTOCHAT_MAX_ROUNDS: usize = crate::autochat::AUTOCHAT_MAX_ROUNDS;
pub const AUTOCHAT_MAX_DYNAMIC_SPAWNS: usize = crate::autochat::AUTOCHAT_MAX_DYNAMIC_SPAWNS;
pub const AUTOCHAT_SPAWN_CHECK_MIN_CHARS: usize = crate::autochat::AUTOCHAT_SPAWN_CHECK_MIN_CHARS;
pub const AUTOCHAT_RLM_THRESHOLD_CHARS: usize = crate::autochat::AUTOCHAT_RLM_THRESHOLD_CHARS;
pub const AUTOCHAT_RLM_FALLBACK_CHARS: usize = crate::autochat::AUTOCHAT_RLM_FALLBACK_CHARS;
pub const AUTOCHAT_RLM_HANDOFF_QUERY: &str = crate::autochat::AUTOCHAT_RLM_HANDOFF_QUERY;
pub const AUTOCHAT_QUICK_DEMO_TASK: &str = crate::autochat::AUTOCHAT_QUICK_DEMO_TASK;
pub const AUTOCHAT_LOCAL_DEFAULT_MODEL: &str = "local_cuda/qwen3.5-9b";
pub const GO_SWAP_MODEL_GLM: &str = "zai/glm-5";
pub const GO_SWAP_MODEL_MINIMAX: &str = "minimax-credits/MiniMax-M2.5-highspeed";
pub const CHAT_SYNC_DEFAULT_INTERVAL_SECS: u64 = 15;
pub const CHAT_SYNC_MAX_INTERVAL_SECS: u64 = 300;
pub const CHAT_SYNC_MAX_BATCH_BYTES: usize = 512 * 1024;
pub const CHAT_SYNC_DEFAULT_BUCKET: &str = "codetether-chat-archive";
pub const CHAT_SYNC_DEFAULT_PREFIX: &str = "sessions";
pub const AGENT_AVATARS: [&str; 12] = [
    "[o_o]", "[^_^]", "[>_<]", "[._.]", "[+_+]", "[~_~]", "[x_x]", "[0_0]", "[*_*]", "[=_=]",
    "[T_T]", "[u_u]",
];
