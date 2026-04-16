//! Tab-management actions.

use super::super::helpers::require_index;
use super::super::http::{get, post};
use super::{Ctx, Outcome};
use anyhow::Result;
use serde_json::json;

pub(in crate::tool::browserctl) async fn tabs(ctx: &Ctx<'_>) -> Result<Outcome> {
    let (status, body) = get(ctx.client, ctx.base_url, "/tabs", ctx.token).await?;
    Ok(("tabs", "/tabs", status, body))
}

pub(in crate::tool::browserctl) async fn tabs_select(ctx: &Ctx<'_>) -> Result<Outcome> {
    let payload = json!({"index": require_index(ctx.input.index, "index")?});
    let (status, body) = post(ctx.client, ctx.base_url, "/tabs/select", ctx.token, payload).await?;
    Ok(("tabs_select", "/tabs/select", status, body))
}

pub(in crate::tool::browserctl) async fn tabs_new(ctx: &Ctx<'_>) -> Result<Outcome> {
    let payload = match ctx.input.url.as_deref().filter(|s| !s.trim().is_empty()) {
        Some(url) => json!({"url": url}),
        None => json!({}),
    };
    let (status, body) = post(ctx.client, ctx.base_url, "/tabs/new", ctx.token, payload).await?;
    Ok(("tabs_new", "/tabs/new", status, body))
}

pub(in crate::tool::browserctl) async fn tabs_close(ctx: &Ctx<'_>) -> Result<Outcome> {
    let payload = match ctx.input.index {
        Some(index) => json!({"index": index}),
        None => json!({}),
    };
    let (status, body) = post(ctx.client, ctx.base_url, "/tabs/close", ctx.token, payload).await?;
    Ok(("tabs_close", "/tabs/close", status, body))
}
