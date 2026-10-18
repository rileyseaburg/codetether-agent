//! Interception and deferred continuation for human goal/task commands.
//!
//! [intercept] handles goal and task commands before session borrowing. [restart]
//! records a one-shot intent; [drain] resumes it only when ownership is available.
mod execute;
mod host;
mod pending;
mod restart_needed;
mod resume;
mod tasks;
use codetether_slash_command::{command_with_optional_args, normalize_slash_command};
pub use host::{Actions, Host, View};
pub use pending::take as take_pending;
pub use resume::{Restart, Resume, drain, restart};
/// Intercept goal/task input without borrowing the active session.
/// # Arguments
/// `app` supplies input, session identity and native UI/command operations.
/// # Returns
/// True when the input was a handled goal/task command.
/// # Examples
/// ```no_run
/// async fn input<H: codetether_goal_command::input::Host>(host: &mut H) {
///     codetether_goal_command::input::intercept(host).await;
/// }
/// ```
pub async fn intercept<H: Host>(app: &mut H) -> bool {
    let input = app.input().trim().to_string();
    let normalized = normalize_slash_command(&input);
    if command_with_optional_args(&normalized, "/go").is_some() {
        execute::run(app, &normalized).await;
        return true;
    }
    if let Some(rest) = command_with_optional_args(&normalized, "/tasks") {
        tasks::show(app, rest).await;
        app.clear();
        return true;
    }
    let Some(rest) = command_with_optional_args(&normalized, "/goal") else {
        return false;
    };
    if rest.trim() == "edit" {
        let id = app.session_id();
        app.edit(&id);
        return true;
    }
    let id = app.session_id();
    let verb = rest.split_whitespace().next().unwrap_or_default();
    let changed = app.goal(&id, rest).await;
    app.history(input);
    app.clear();
    if changed && restart_needed::for_verb(verb) {
        app.restart(&id);
    }
    app.redraw();
    true
}
