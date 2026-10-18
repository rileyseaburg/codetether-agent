//! Remote worker resource preparation and agent-loop argument mapping.
//!
//! [run] binds the remote process lifecycle to the shared agent loop.
//! [RemoteHost] supplies provider loading, workspace tools and session instructions.
mod adapter;
mod host;
mod invoke;
mod resources;
pub use host::RemoteHost;

/// Run a remote worker using its encoded payload or payload environment variable.
/// # Arguments
/// `payload_base64` and `payload_env` retain the remote runtime's source precedence.
/// # Returns
/// Success after the remote runtime emits the structured task result.
/// # Errors
/// Returns payload, workspace, provider, agent execution or serialization errors.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_swarm_agent::remote::RemoteHost>() -> anyhow::Result<()> {
/// codetether_swarm_agent::remote::run::<H>(None, "TASK_PAYLOAD".into()).await
/// # }
/// ```
pub async fn run<H: RemoteHost>(
    payload_base64: Option<String>,
    payload_env: String,
) -> anyhow::Result<()> {
    codetether_swarm_remote_runtime::run::<adapter::Adapter<H>>(payload_base64, payload_env).await
}
