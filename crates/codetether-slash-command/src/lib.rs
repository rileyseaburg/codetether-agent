//! Slash-command parsing, discovery and user-facing descriptions.
//!
//! Use `normalize_slash_command` and `normalize_easy_command` before dispatch,
//! `command_with_optional_args` for exact command boundaries, and `suggestions`,
//! `usage_hint` or `match_slash_command_hint` for completion interfaces.
mod arguments;
mod catalog;
mod completion;
mod descriptions;
mod easy;
mod hint_match;
mod normalize;
pub mod routing;
mod usage;
pub use arguments::command_with_optional_args;
pub use catalog::SLASH_COMMANDS;
pub use completion::suggestions;
pub use descriptions::SLASH_HINTS;
pub use easy::normalize_easy_command;
pub use hint_match::match_slash_command_hint;
pub use normalize::normalize_slash_command;
pub use usage::usage_hint;
#[cfg(test)]
mod tests;

mod completion_navigation;
mod completion_refresh;
mod completion_state;
mod input_hint;
pub use completion_state::CompletionState;
pub use input_hint::current_slash_hint;

/// Commands available without mutable session access.
pub mod session_free;

/// Interactive command sequencing with native execution callbacks.
pub mod pipeline;
