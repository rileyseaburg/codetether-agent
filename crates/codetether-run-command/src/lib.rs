//! Pure input policy for non-interactive go/team and relay commands.
//!
//! [normalize_cli_go_command] expands aliases; [validate_easy_go_task] checks the
//! task text and [resolve_autochat_model] applies the existing model precedence.
mod boundary;
mod detect;
mod model;
mod normalize;
mod task;
pub use boundary::command_with_optional_args;
pub use detect::is_easy_go_command;
pub use model::resolve_autochat_model;
pub use normalize::normalize_cli_go_command;
pub use task::{normalize_go_task_input, validate_easy_go_task};
#[cfg(test)]
mod tests;
