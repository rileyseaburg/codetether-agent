//! Pure settings labels and cycle policies, independent of UI and persistence.
//!
//! Call these functions with current provider values, then let the application
//! apply the returned setting and persist its session. Provider-specific cycles
//! remain distinct, including their handling of unknown overrides.
mod access;
mod bedrock;
mod boolean;
pub mod command;
mod reasoning;
pub use access::{access_label, next_access_mode};
pub use bedrock::{
    bedrock_service_tier_label, bedrock_thinking_effort_label, next_bedrock_service_tier,
    next_bedrock_thinking_effort,
};
pub use boolean::{
    auto_apply_status_message, autocomplete_status_message, network_access_status_message,
    on_off_label, worktree_status_message,
};
pub use reasoning::{next_codex_effort, next_openrouter_effort, reasoning_label};
