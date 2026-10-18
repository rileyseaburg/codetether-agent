//! Native services for the shared parallel batch lifecycle.
use crate::provider::Provider;
use crate::session::{Session, SessionEvent};
use crate::tool::ToolRegistry;
use std::{path::Path, sync::Arc};
use tokio::sync::mpsc;
pub(crate) async fn try_execute(
    session: &mut Session,
    calls: &[(String, String, serde_json::Value)],
    registry: &ToolRegistry,
    cwd: &Path,
    model: &str,
    provider: Arc<dyn Provider>,
    event_tx: &mpsc::Sender<SessionEvent>,
    no_match_count: &mut u32,
) -> bool {
    let mut session = super::host::NativeHost {
        session,
        cwd,
        model,
        provider,
    };
    let execution = super::super::tool_exec::NativeHost { registry };
    codetether_tool_parallel::try_execute(&mut session, calls, &execution, event_tx, no_match_count)
        .await
}
