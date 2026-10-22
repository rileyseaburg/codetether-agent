//! Attachment-aware finalization of a normally routed chat prompt.
//!
//! [finish] consumes attachments before checking empty input, expands the agent
//! prompt, adopts a goal, publishes expanded chat content and dispatches it.
mod actions;
mod attachments;
mod binding;
mod run;
pub use actions::Actions;
pub use attachments::Attachments;
pub use run::finish;
