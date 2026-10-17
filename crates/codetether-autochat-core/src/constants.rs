//! Auto-chat bounds, prompt templates and protocol markers.

pub const AUTOCHAT_MAX_AGENTS: usize = 8;

pub const AUTOCHAT_DEFAULT_AGENTS: usize = 3;

pub const AUTOCHAT_MAX_ROUNDS: usize = 3;

pub const AUTOCHAT_MAX_DYNAMIC_SPAWNS: usize = 3;

pub const AUTOCHAT_SPAWN_CHECK_MIN_CHARS: usize = 800;

pub const AUTOCHAT_RLM_THRESHOLD_CHARS: usize = 6_000;

pub const AUTOCHAT_RLM_FALLBACK_CHARS: usize = 3_500;

pub const AUTOCHAT_STATUS_MAX_ROUNDS_REACHED: &str = "max_rounds_reached";

pub const AUTOCHAT_NO_PRD_FLAG: &str = "--no-prd";

pub const AUTOCHAT_RLM_HANDOFF_QUERY: &str = "Prepare a concise relay handoff for the next specialist.\n\
Return FINAL(JSON) with this exact shape:\n\
{\"kind\":\"semantic\",\"file\":\"relay_handoff\",\"answer\":\"...\"}\n\
The \"answer\" must include:\n\
1) key conclusions,\n\
2) unresolved risks,\n\
3) one exact next action.\n\
Keep it concise and actionable.";

pub const AUTOCHAT_QUICK_DEMO_TASK: &str = "Self-organize into the right specialties for this task, then relay one concrete implementation plan with clear next handoffs.";

pub(crate) const REQUIRED_RELAY_CAPABILITIES: [&str; 4] =
    ["relay", "context-handoff", "rlm-aware", "autochat"];
