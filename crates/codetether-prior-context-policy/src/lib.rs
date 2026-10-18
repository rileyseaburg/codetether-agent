//! Prior-context directive interpretation shared by session runtimes.
//!
//! [resolve] applies one-turn directives to the current user turn and preserves
//! the newest persistent directive across follow-ups. [classify] interprets one
//! message; [latest_persistent] supplies the durable baseline.
mod actions;
mod denial;
pub mod directive;
mod directive_clause;
mod directive_message;
mod example_line;
mod normalize;
mod normalize_line;
mod permission;
mod permission_action;
mod permission_target;
mod quoted;
mod quoted_target;
mod reenable;
mod source_preference;
mod targets;
pub use directive::{
    Directive, latest_persistent, persistent_update, resolve, resolve_with_default,
};
pub use directive_message::classify;
#[cfg(test)]
mod tests;
