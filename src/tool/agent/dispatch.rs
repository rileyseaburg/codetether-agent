//! Routing from parsed legacy agent actions to focused handlers.

use super::params::Params;
use crate::tool::ToolResult;
use anyhow::{Context, Result};

#[path = "mux.rs"]
mod mux;

pub(super) async fn execute(params: &Params) -> Result<ToolResult> {
    let owner = params.parent_session_id.as_deref();
    match params.action.as_str() {
        "spawn" => super::spawn::handle_spawn(params).await,
        "message" => super::message::handle_message(params).await,
        "list" => super::handlers::handle_list(owner).await,
        "read" => mux::read(target(params, "read")?).await,
        "interact" => mux::interact(target(params, "interact")?).await,
        "status" => status(params, owner).await,
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

async fn status(params: &Params, owner: Option<&str>) -> Result<ToolResult> {
    if let Some(name) = params.name.as_deref()
        && let Some(result) = mux::status(name).await?
    {
        return Ok(result);
    }
    Ok(super::status::handle_status(owner))
}

fn target<'a>(params: &'a Params, action: &str) -> Result<&'a str> {
    params
        .name
        .as_deref()
        .with_context(|| format!("name required for {action}"))
}
