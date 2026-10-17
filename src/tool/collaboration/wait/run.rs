//! Application integration-test adapter for shared wait dispatch.
pub(super) async fn execute(args: super::Args) -> anyhow::Result<crate::tool::ToolResult> {
    codetether_collaboration_tools::execute_wait::<super::super::host::ApplicationHost>(args).await
}
