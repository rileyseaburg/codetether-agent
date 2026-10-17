//! Text and request policies for the legacy RLM executor.
//!
//! [context_summary] and [system_prompt] describe the loaded REPL context.
//! [subquery_request] shares truncation, messages, and generation settings
//! across the DSL and structured-tool sub-query paths.
mod context_summary;
mod subquery_context;
mod subquery_messages;
mod subquery_request;
mod system_prompt;
pub use context_summary::context_summary;
pub use subquery_request::subquery_request;
pub use system_prompt::system_prompt;
