//! Agent display profiles, deterministic avatars, and relay labels.
//!
//! Use [agent_profile] for persona metadata or [format_agent_identity]
//! for a compact display label. This crate owns no runtime agent state.
mod avatar_glyphs;
mod avatars;
mod fallback_profiles;
mod fallback_select;
mod format;
mod named_delivery;
mod named_planning;
mod profile;
mod profiles;
mod relay_format;
pub use avatar_glyphs::AGENT_AVATARS;
pub use avatars::agent_avatar;
pub use format::{format_agent_identity, format_agent_profile_summary};
pub use profile::AgentProfile;
pub use profiles::agent_profile;
pub use relay_format::{format_relay_handoff_line, format_relay_participant};

mod mission;
pub use mission::system_prompt;
#[cfg(test)]
mod mission_tests;
