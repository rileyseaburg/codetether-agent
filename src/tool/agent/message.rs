//! Handle the "message" action.

use super::helpers;
use crate::tool::ToolResult;
use anyhow::Result;

#[path = "remote/mod.rs"]
pub(super) mod remote;
#[path = "message/remote_fallback.rs"]
pub(in crate::tool::agent) mod remote_fallback;
#[path = "message/run.rs"]
pub(super) mod run;
#[path = "message/start.rs"]
pub(in crate::tool::agent) mod start;
#[path = "message/session.rs"]
mod task_session;
/// Route a message through shared child-turn coordination.
pub(super) async fn handle_message(params: &helpers::Params) -> Result<ToolResult> {
    codetether_child_turn::handle_message::<super::turn_host::ApplicationHost>(params).await
}
