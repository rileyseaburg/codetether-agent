//! Handle the "spawn" action.

use super::store::{self, AgentEntry};
use super::helpers;
use super::policy;
use crate::tool::ToolResult;
use anyhow::{Context, Result};

pub(super) async fn handle_spawn(params: &helpers::Params) -> Result<ToolResult> {
    let name = params.name.as_ref().context("name required for spawn")?;
    let instructions = params
        .instructions
        .as_ref()
        .context("instructions required for spawn")?;
    let requested = params.model.as_ref().context("model required for spawn")?;
    let current = params
        .current_model
        .clone()
        .or_else(|| std::env::var("CODETETHER_DEFAULT_MODEL").ok())
        .context("cannot determine caller model")?;

    if policy::normalize_model(requested) == policy::normalize_model(&current) {
        return Ok(ToolResult::error(format!(
            "Spawn blocked: '{requested}' must differ from '{current}'."
        )));
    }

    let registry = helpers::get_registry().await?;
    if !policy::is_free_or_eligible(requested, &registry).await {
        return Ok(ToolResult::error(format!(
            "Spawn blocked: '{requested}' is not free/subscription-eligible. \
             Use ':free' models, subscription providers, or OAuth providers."
        )));
    }

    if store::contains(name) {
        return Ok(ToolResult::error(format!("Agent @{name} exists. Use kill first.")));
    }

    let mut session = helpers::create_agent_session(name, instructions, requested).await?;
    if let Err(e) = session.save().await {
        tracing::warn!(agent = %name, error = %e, "Failed to save spawned agent session");
    }
    store::insert(name.clone(), AgentEntry { instructions: instructions.clone(), session });
    tracing::info!(agent = %name, "Sub-agent spawned");
    Ok(ToolResult::success(format!("Spawned @{name} on '{requested}': {instructions}")))
}
