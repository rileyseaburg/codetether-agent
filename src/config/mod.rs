//! Configuration system
//!
//! Handles global, project, and environment-backed settings.

pub mod guardrails;

mod a2a;
mod access_mode;
mod access_mode_effective;
mod access_mode_override;
mod access_mode_parse;
mod access_mode_runtime;
mod agent;
mod agents;
mod approval;
mod bool_parse;
mod core;
mod default;
mod env;
mod env_agents;
mod legacy;
mod load;
mod load_workspace;
mod lsp;
mod merge;
mod merge_policy;
mod merge_sections;
mod path;
mod permission;
mod policy_accessors;
mod policy_raw;
mod policy_requirement_accessors;
mod policy_trust_accessors;
mod profile;
mod profile_policy;
mod project_policy;
mod provider;
mod reexports;
mod requirements;
mod requirements_pick;
mod sandbox;
mod session;
mod set;
mod set_global;
mod set_parse;
mod set_value;
mod telemetry;
mod trust;
mod trust_status;
pub mod trust_store;
mod ui;

pub use reexports::*;

#[cfg(test)]
mod test_modules;
