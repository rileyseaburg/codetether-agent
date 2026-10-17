//! Bind concrete search backends to the application's memory factory.
use super::types::BackendChoice;
/// Execute one backend choice with the existing automatic embedding selection.
///
/// # Arguments
/// * `choice` - Backend selection and supplied tool arguments.
/// * `query` - Query used to fill missing arguments.
/// # Returns
/// The selected backend's tool result.
/// # Errors
/// Returns errors raised by the selected backend.
/// # Examples
/// ```rust,no_run
/// # async fn example(choice: &codetether_agent::search::BackendChoice) -> anyhow::Result<()> {
/// let _result = codetether_agent::search::dispatch::run_choice(choice, "main").await?;
/// # Ok(()) }
/// ```
pub async fn run_choice(
    choice: &BackendChoice,
    query: &str,
) -> anyhow::Result<crate::tool::ToolResult> {
    codetether_search_router::dispatch::run_choice::<crate::tool::memory::ApplicationAutoEmbedder>(
        choice, query,
    )
    .await
}
