//! Session-command recognition after conditional relay dispatch.
use super::SessionCommand;
/// Recognize session commands using the existing priority and boundary rules.
/// # Arguments
/// * `normalized` - Input after command normalization.
/// # Returns
/// Command kind and trimmed arguments, or none when this stage does not handle it.
/// # Examples
/// ```
/// assert!(codetether_slash_command::routing::session("/forked").is_none());
/// ```
pub fn session(normalized: &str) -> Option<(SessionCommand, &str)> {
    for (name, kind) in [
        ("/forage", SessionCommand::Forage),
        ("/goal", SessionCommand::Goal),
        ("/a2a", SessionCommand::A2a),
        ("/undo", SessionCommand::Undo),
        ("/detach", SessionCommand::Detach),
        ("/fork", SessionCommand::Fork),
    ] {
        if let Some(rest) = crate::command_with_optional_args(normalized, name) {
            return Some((kind, rest));
        }
    }
    None
}
