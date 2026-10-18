//! Shared one-shot execution specialized with native application services.
use crate::tool::ToolResult;
use crate::tool::agent::spawn_request::SpawnRequest;
pub(super) async fn run(
    request: &SpawnRequest<'_>,
    warning: Option<&str>,
) -> anyhow::Result<ToolResult> {
    codetether_agent_spawn::run_ephemeral::<super::ephemeral_host::ApplicationHost>(
        request, warning,
    )
    .await
}
