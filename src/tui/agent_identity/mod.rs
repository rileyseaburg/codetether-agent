//! Compatibility exports for agent labels and text previews.
pub use codetether_agent_identity::{
    AgentProfile, agent_avatar, agent_profile, format_agent_identity, format_agent_profile_summary,
    format_relay_handoff_line, format_relay_participant,
};
pub use codetether_tool_preview::{build_tool_arguments_preview, format_tool_call_arguments};
pub use codetether_util::text_preview::build_text_preview;
