//! Routing from parsed legacy agent actions to focused handlers.

use super::params::Params;
use crate::tool::ToolResult;
use anyhow::{Context, Result};

pub(super) async fn execute(params: &Params) -> Result<ToolResult> {
    let owner = params.parent_session_id.as_deref();
    match params.action.as_str() {
        "spawn" => super::spawn::handle_spawn(params).await,
        "message" => super::message::handle_message(params).await,
        "list" => Ok(super::handlers::handle_list(owner)),
        "status" => Ok(super::status::handle_status(owner)),
        "interrupt" => super::actions::execute_interrupt(params).await,
        "close" => super::thread_lifecycle::close(target(params, "close")?, owner).await,
        "resume" => {
            super::thread_lifecycle::resume(
                target(params, "resume")?,
                owner,
                params.resume_config(),
            )
            .await
        }
        "kill" => super::actions::execute_kill(params).await,
        _ => Ok(super::actions::unknown_action_result(&params.action)),
    }
}

fn target<'a>(params: &'a Params, action: &str) -> Result<&'a str> {
    params
        .name
        .as_deref()
        .with_context(|| format!("name required for {action}"))
}
