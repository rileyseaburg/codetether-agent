//! Agent identity formatting and avatar mapping.

mod avatars;
mod fallback_profiles;
mod format;
mod named_profiles;
mod profiles;

pub use avatars::agent_avatar;
pub use format::{format_agent_identity, format_agent_profile_summary};
pub use profiles::{AgentProfile, agent_profile};
