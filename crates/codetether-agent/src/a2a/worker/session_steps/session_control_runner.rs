//! Invoke the real worker loop with a bounded mock and no registered tools.

use super::Mock;
use crate::{provider::Provider, session::Session};
use std::sync::Arc;

pub(super) async fn run(
    session: &mut Session,
    provider: Arc<Mock>,
    steps: usize,
) -> anyhow::Result<super::super::SessionStepsOutcome> {
    let tools = crate::tool::ToolRegistry::new();
    let workspace = tempfile::tempdir()?;
    let provider: Arc<dyn Provider> = provider;
    tokio::time::timeout(
        // This includes durable SQLite writes on shared CI storage, not just cancellation.
        std::time::Duration::from_secs(30),
        super::super::run_session_steps(
            provider,
            session,
            "test",
            "Test system",
            &tools,
            &[],
            super::super::super::AutoApprove::All,
            workspace.path(),
            None,
            steps,
        ),
    )
    .await
    .expect("worker control must not stall")
}
