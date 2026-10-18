//! Native authorization, registry restrictions and autonomous prompts.
//!
//! [system_prompt] appends the same session goal-maintenance instructions.
//! [restrict_registry] applies shared autonomous/read-only tool restrictions.
mod prompts;
mod registry;
mod runtime_gate;
pub use codetether_swarm_core::prompt_policy::SystemPromptInput;
pub use prompts::system_prompt;
pub use registry::restrict_registry;
pub use runtime_gate::runtime_denial;
