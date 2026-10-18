//! Forage execution from an A2A worker task.
//!
//! [`handle`] builds command options through the shared worker metadata policy,
//! runs forage using the caller's execution host and renders the final summary.
use crate::Host;

/// Execute a forage task and return its textual run summary.
/// # Arguments
/// `title` and `prompt` supply task intent, `metadata` supplies execution options,
/// and `selected_model` is the worker's model override. `H` binds execution services.
/// # Returns
/// The same text rendering used by the forage run summary.
/// # Errors
/// Propagates forage setup, discovery and execution errors.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_forage_runtime::Host>() -> anyhow::Result<()> {
/// let _summary = codetether_forage_runtime::worker_task::handle::<H>(
///     "task", "mission", &serde_json::Map::new(), None).await?;
/// # Ok(()) }
/// ```
pub async fn handle<H: Host>(
    title: &str,
    prompt: &str,
    metadata: &serde_json::Map<String, serde_json::Value>,
    selected_model: Option<String>,
) -> anyhow::Result<String> {
    let args = codetether_cli_options::forage_task::build_forage_args(
        prompt,
        title,
        metadata,
        selected_model,
    );
    Ok(crate::execute_with_summary::<H>(args).await?.render_text())
}
