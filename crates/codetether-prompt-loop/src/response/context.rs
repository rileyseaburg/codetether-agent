use codetether_provider_core::{Message, ToolDefinition};
use std::path::Path;
/// Current inputs to build-mode response guards.
/// # Examples
/// ```
/// let context = codetether_prompt_loop::response::BuildContext {
///     agent: "build", messages: &[], tools: &[], cwd: std::path::Path::new("."),
/// };
/// assert!(context.messages.is_empty());
/// ```
pub struct BuildContext<'a> {
    /// Current agent persona.
    pub agent: &'a str,
    /// Current conversation transcript.
    pub messages: &'a [Message],
    /// Tool definitions offered to the provider.
    pub tools: &'a [ToolDefinition],
    /// Workspace used to interpret the request.
    pub cwd: &'a Path,
}
