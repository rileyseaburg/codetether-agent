//! Native provider, agent and workspace composition for independent review.
use codetether_goal_verification::{ReviewExecution, VerificationRequest};
use std::path::Path;
/// Shared verifier runtime using native providers and application tools T.
/// # Examples
/// ```
/// let _: Option<codetether_native_goal_tools::verification::Runtime<()>> = None;
/// ```
pub type Runtime<T> = codetether_goal_verification::runtime::application::ApplicationHost<
    codetether_native_providers::registry::application::Native,
    T,
    codetether_native_swarm::AgentHost<T>,
    codetether_native_swarm::workspace::WorkspaceHost<T>,
>;
/// Run an independent review with the application's native tool catalog.
/// # Arguments
/// * worker_model - Original worker model used by verifier model selection.
/// * workspace - Directory inspected by the verifier.
/// * request - Persisted goal requirements and claimed result.
/// # Returns
/// The report and captured verifier execution identity, including failure reports.
/// # Examples
/// ```no_run
/// # async fn example<T: codetether_application_registry::Application>(request: &codetether_goal_verification::VerificationRequest) {
/// let result = codetether_native_goal_tools::verification::review::<T>(
///     None, std::path::Path::new("."), request,
/// ).await;
/// # }
/// ```
pub async fn review<T: codetether_application_registry::Application>(
    worker_model: Option<&str>,
    workspace: &Path,
    request: &VerificationRequest,
) -> ReviewExecution {
    codetether_goal_verification::runtime::review::<Runtime<T>>(worker_model, workspace, request)
        .await
}
