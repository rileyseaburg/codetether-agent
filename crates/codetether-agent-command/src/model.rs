//! Current model snapshot for newly spawned agents.
/// Select the last completed model, then the currently selected picker model.
/// # Arguments
/// Receives the last model, available model list and selected list index.
/// # Returns
/// An owned model identifier, or none when neither source is available.
/// # Examples
/// ```
/// let model = codetether_agent_command::model::current(None, &["one".into()], 0);
/// assert_eq!(model.as_deref(), Some("one"));
/// ```
pub fn current(last: Option<&String>, available: &[String], selected: usize) -> Option<String> {
    last.cloned().or_else(|| available.get(selected).cloned())
}
