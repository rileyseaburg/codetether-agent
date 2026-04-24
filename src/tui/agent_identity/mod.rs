//! Agent identity formatting and avatar mapping.

mod avatars;
mod fallback_profiles;
mod format;
mod named_profiles;
mod profiles;
mod relay_format;
mod text_preview;
mod tool_args;

pub use avatars::agent_avatar;
pub use format::{format_agent_identity, format_agent_profile_summary};
pub use profiles::{AgentProfile, agent_profile};
pub use relay_format::{format_relay_handoff_line, format_relay_participant};
pub use text_preview::build_text_preview;
pub use tool_args::{build_tool_arguments_preview, format_tool_call_arguments};
