//! Application specialization of durable idle-child delivery.
pub(crate) async fn queue_only(
    target: &str,
    owner: Option<&str>,
    message: String,
) -> anyhow::Result<crate::tool::ToolResult> {
    codetether_agent_communication::queue_only::<super::host::ApplicationHost>(
        target, owner, message,
    )
    .await
}
