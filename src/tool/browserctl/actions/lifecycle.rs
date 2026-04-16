//! Lifecycle actions: wait + screenshot.

use super::super::helpers::{optional_string, require_string};
use super::super::http::post;
use super::{Ctx, Outcome};
use anyhow::Result;
use serde_json::json;

pub(in crate::tool::browserctl) async fn wait(ctx: &Ctx<'_>) -> Result<Outcome> {
    let payload = json!({
        "text": optional_string(&ctx.input.text),
        "text_gone": optional_string(&ctx.input.text_gone),
        "url_contains": optional_string(&ctx.input.url_contains),
        "selector": optional_string(&ctx.input.selector),
        "frame_selector": optional_string(&ctx.input.frame_selector),
        "state": ctx.input.state.clone().unwrap_or_else(|| "visible".into()),
        "timeout_ms": ctx.input.timeout_ms.unwrap_or(5_000),
    });
    let (status, body) = post(ctx.client, ctx.base_url, "/wait", ctx.token, payload).await?;
    Ok(("wait", "/wait", status, body))
}

pub(in crate::tool::browserctl) async fn screenshot(ctx: &Ctx<'_>) -> Result<Outcome> {
    let payload = json!({
        "path": require_string(&ctx.input.path, "path")?,
        "full_page": ctx.input.full_page.unwrap_or(true),
        "selector": optional_string(&ctx.input.selector),
        "frame_selector": optional_string(&ctx.input.frame_selector),
    });
    let (status, body) = post(ctx.client, ctx.base_url, "/screenshot", ctx.token, payload).await?;
    Ok(("screenshot", "/screenshot", status, body))
}
