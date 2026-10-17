//! Auto-chat application adapters and shared policy exports.
pub mod delegation;
pub mod model_rotation;
pub mod shared_context;
pub mod transport;
pub use codetether_autochat_core::{
    AUTOCHAT_DEFAULT_AGENTS, AUTOCHAT_MAX_AGENTS, AUTOCHAT_MAX_DYNAMIC_SPAWNS, AUTOCHAT_MAX_ROUNDS,
    AUTOCHAT_NO_PRD_FLAG, AUTOCHAT_QUICK_DEMO_TASK, AUTOCHAT_RLM_FALLBACK_CHARS,
    AUTOCHAT_RLM_HANDOFF_QUERY, AUTOCHAT_RLM_THRESHOLD_CHARS, AUTOCHAT_SPAWN_CHECK_MIN_CHARS,
    AUTOCHAT_STATUS_MAX_ROUNDS_REACHED, ParsedAutochatRequest, ensure_required_relay_capabilities,
    normalize_for_convergence, parse_autochat_args, parse_autochat_request,
};
