//! Dispatch UI-only commands while the current session is checked out.
//!
//! Implement [Host] for native attachments and view operations. [run] returns
//! false for commands that require a session so callers can report it as busy.
mod binding;
mod host;
pub use host::Host;
/// Normalize and dispatch a command that does not require session access.
///
/// # Arguments
/// * `app` — Native UI operations.
/// * `prompt` — Raw slash-command input.
/// # Returns
/// True only for file, image, settings, model or help commands handled here.
/// # Examples
/// ```no_run
/// # fn command(app: &mut impl codetether_slash_command::session_free::Host) {
/// assert!(codetether_slash_command::session_free::run(app, "/help"));
/// # }
/// ```
pub fn run(app: &mut impl Host, prompt: &str) -> bool {
    let normalized = crate::normalize_slash_command(prompt);
    if let Some(rest) = crate::command_with_optional_args(&normalized, "/file") {
        app.attach_file(rest.trim().trim_matches(|c| c == '"' || c == '\''));
        return true;
    }
    if let Some(rest) = crate::command_with_optional_args(&normalized, "/image") {
        app.attach_image(rest.trim().trim_matches(|c| c == '"' || c == '\''));
        return true;
    }
    match normalized.as_str() {
        "/settings" => app.open_settings(),
        "/model" => app.open_model(),
        "/help" => app.open_help(),
        _ => return false,
    }
    true
}
