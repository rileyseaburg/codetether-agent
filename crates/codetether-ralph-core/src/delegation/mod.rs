//! Shared relay-role mapping and calibrated provider selection.
//!
//! [choose_provider_for_role] applies delegation state to lazily supplied
//! provider names. Registry construction and credential access belong to the host.
use codetether_context_policy::delegation::{DelegationState, skills as delegation_skills};
use codetether_context_policy::relevance::{Bucket, Dependency, Difficulty, ToolUse};
mod difficulty;
mod roles;
mod select;
mod skill;
mod tool_use;
use difficulty::difficulty_for_role;
pub use roles::*;
pub use select::choose_provider_for_role;
use skill::skill_for_role;
use tool_use::tool_use_for_role;
