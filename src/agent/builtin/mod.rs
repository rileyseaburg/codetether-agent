//! Built-in agent definitions and prompts.
//!
//! This module exposes the built-in agent catalog plus system-prompt builders
//! used by sessions, Ralph, and swarm execution.
//!
//! # Examples
//!
//! ```ignore
//! let prompt = codetether::agent::builtin::build_system_prompt(std::path::Path::new("."));
//! ```

mod agents_md;
mod definitions;
mod prompts;
mod system_prompt;

#[cfg(test)]
mod tests;

#[allow(unused_imports)]
pub use agents_md::{load_agents_md, load_all_agents_md};
pub use definitions::{build_agent, explore_agent, plan_agent};
#[allow(unused_imports)]
pub use prompts::{BUILD_SYSTEM_PROMPT, EXPLORE_SYSTEM_PROMPT, PLAN_SYSTEM_PROMPT};
#[allow(unused_imports)]
pub use system_prompt::{build_plan_system_prompt, build_system_prompt};
