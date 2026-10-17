//! Recursive Language Model tool for bounded content collection and analysis.
//!
//! Construct [RlmTool] with an application [Host], provider, model and RLM config.
//! The tool validates actions, reads bounded file inputs, and returns either
//! the processed response or the existing deterministic truncation fallback.
mod collect;
mod context;
mod execute;
mod fallback;
mod host;
mod process;
mod schema;
mod tool;
mod tool_impl;
pub use host::Host;
pub use tool::RlmTool;
