//! Publish human prompts with workspace and session provenance.
//! [publish] emits a correlated UserPrompt when a bus is present. [Context]
//! supplies session metadata; an absent workspace uses the current directory.
mod context;
mod publish;
pub use context::Context;
pub use publish::publish;
