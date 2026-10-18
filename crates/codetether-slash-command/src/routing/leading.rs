//! Highest-priority interactive command recognition.
use super::LeadingCommand;
/// Recognize leading commands using exact command boundaries in existing order.
/// # Arguments
/// * `normalized` - Input after easy-alias and slash normalization.
/// # Returns
/// Command kind and trimmed arguments, or none for a later dispatch stage.
/// # Examples
/// ```
/// assert!(codetether_slash_command::routing::leading("/image-extra").is_none());
/// ```
pub fn leading(normalized: &str) -> Option<(LeadingCommand, &str)> {
    for (name, kind) in [
        ("/go", LeadingCommand::Go),
        ("/image", LeadingCommand::Image),
        ("/file", LeadingCommand::File),
        ("/edit", LeadingCommand::Edit),
        ("/autoapply", LeadingCommand::AutoApply),
        ("/network", LeadingCommand::Network),
        ("/autocomplete", LeadingCommand::Autocomplete),
        ("/ask", LeadingCommand::Ask),
        ("/context", LeadingCommand::Context),
        ("/mcp", LeadingCommand::Mcp),
    ] {
        if let Some(rest) = crate::command_with_optional_args(normalized, name) {
            return Some((kind, rest));
        }
    }
    None
}
