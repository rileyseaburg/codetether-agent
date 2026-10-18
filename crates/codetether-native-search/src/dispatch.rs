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
/// # async fn example(choice: &codetether_native_search::BackendChoice) -> anyhow::Result<()> {
/// let _result = codetether_native_search::dispatch::run_choice(choice, "main").await?;
/// # Ok(()) }
/// ```
pub async fn run_choice(
    choice: &BackendChoice,
    query: &str,
) -> anyhow::Result<codetether_tool_core::ToolResult> {
    codetether_search_router::dispatch::run_choice::<
        codetether_native_memory::ApplicationAutoEmbedder,
    >(choice, query)
    .await
}
