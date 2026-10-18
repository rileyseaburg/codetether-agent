//! Tool catalog prompts, discovery responses and explicit schema activation.
//!
//! [list_tools_bootstrap_definition] describes the virtual catalog tool.
//! [list_tools_bootstrap_output] returns a filtered list or a full schema;
//! [promote] adds an explicitly requested schema to the advertised tool set.
//! [inject_tool_prompt] supplies fallback markup instructions for tool use.
mod listing;
mod lookup;
mod output;
mod promotion;
mod prompt;
mod schema;
pub use output::list_tools_bootstrap_output;
pub use promotion::promote;
pub use prompt::inject_tool_prompt;
pub use schema::list_tools_bootstrap_definition;
#[cfg(test)]
mod promotion_tests;
