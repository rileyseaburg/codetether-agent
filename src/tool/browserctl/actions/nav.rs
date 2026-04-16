//! Navigation + connection lifecycle actions.

use super::super::helpers::require_string;
use super::super::http::{get, post};
use super::{Ctx, Outcome};
use anyhow::Result;
use serde_json::json;

pub(in crate::tool::browserctl) async fn health(ctx: &Ctx<'_>) -> Result<Outcome> {
    let (status, body) = get(ctx.client, ctx.base_url, "/health", ctx.token).await?;
    Ok(("health", "/health", status, body))
}

pub(in crate::tool::browserctl) async fn start(ctx: &Ctx<'_>) -> Result<Outcome> {
    let payload = json!({
        "headless": ctx.input.headless.unwrap_or(true),
        "executable_path": ctx.input.executable_path.as_deref(),
    });
    let (status, body) = post(ctx.client, ctx.base_url, "/start", ctx.token, payload).await?;
    Ok(("start", "/start", status, body))
}

pub(in crate::tool::browserctl) async fn stop(ctx: &Ctx<'_>) -> Result<Outcome> {
    let (status, body) = post(ctx.client, ctx.base_url, "/stop", ctx.token, json!({})).await?;
    Ok(("stop", "/stop", status, body))
}

pub(in crate::tool::browserctl) async fn snapshot(ctx: &Ctx<'_>) -> Result<Outcome> {
    let (status, body) = get(ctx.client, ctx.base_url, "/snapshot", ctx.token).await?;
    Ok(("snapshot", "/snapshot", status, body))
}

pub(in crate::tool::browserctl) async fn console(ctx: &Ctx<'_>) -> Result<Outcome> {
    let (status, body) = get(ctx.client, ctx.base_url, "/console", ctx.token).await?;
    Ok(("console", "/console", status, body))
}

pub(in crate::tool::browserctl) async fn goto(ctx: &Ctx<'_>) -> Result<Outcome> {
    let payload = json!({
        "url": require_string(&ctx.input.url, "url")?,
        "wait_until": ctx.input.wait_until.clone().unwrap_or_else(|| "domcontentloaded".into()),
    });
    let (status, body) = post(ctx.client, ctx.base_url, "/goto", ctx.token, payload).await?;
    Ok(("goto", "/goto", status, body))
}

pub(in crate::tool::browserctl) async fn back(ctx: &Ctx<'_>) -> Result<Outcome> {
    let (status, body) = post(ctx.client, ctx.base_url, "/back", ctx.token, json!({})).await?;
    Ok(("back", "/back", status, body))
}

pub(in crate::tool::browserctl) async fn reload(ctx: &Ctx<'_>) -> Result<Outcome> {
    let (status, body) = post(ctx.client, ctx.base_url, "/reload", ctx.token, json!({})).await?;
    Ok(("reload", "/reload", status, body))
}
