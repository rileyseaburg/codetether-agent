use super::{json, wire::DecompositionResponse, *};
/// Parse a model response into tasks, resolving dependency names to generated IDs.
///
/// # Arguments
/// * `response` - JSON, fenced JSON, or text surrounding a JSON object.
///
/// # Returns
/// The model's tasks with resolved dependencies; stage validation is separate.
///
/// # Errors
/// Returns an error for malformed JSON or an unknown dependency name.
///
/// # Examples
/// ```
/// let tasks = codetether_swarm_core::planning::parse_decomposition(
///     r#"{"subtasks":[{"name":"inspect","instruction":"read source"}]}"#,
/// ).unwrap();
/// assert_eq!(tasks[0].name, "inspect");
/// ```
pub fn parse_decomposition(response: &str) -> Result<Vec<SubTask>> {
    let json_str = json::extract(response);
    let parsed: DecompositionResponse = serde_json::from_str(json_str.trim())
        .map_err(|e| anyhow::anyhow!("Failed to parse decomposition: {}", e))?;

    super::create::build(parsed)
}
