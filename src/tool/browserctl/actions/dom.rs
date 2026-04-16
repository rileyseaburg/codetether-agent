//! DOM interaction actions: click, fill, type, press, text/html, click_text, fill_native, toggle.

use super::super::helpers::{optional_string, require_string};
use super::super::http::post;
use super::{Ctx, Outcome};
use anyhow::Result;
use serde_json::json;

pub(in crate::tool::browserctl) async fn click(ctx: &Ctx<'_>) -> Result<Outcome> {
    let payload = json!({
        "selector": require_string(&ctx.input.selector, "selector")?,
        "frame_selector": optional_string(&ctx.input.frame_selector),
    });
    let (status, body) = post(ctx.client, ctx.base_url, "/click", ctx.token, payload).await?;
    Ok(("click", "/click", status, body))
}

pub(in crate::tool::browserctl) async fn fill(ctx: &Ctx<'_>) -> Result<Outcome> {
    let payload = json!({
        "selector": require_string(&ctx.input.selector, "selector")?,
        "value": require_string(&ctx.input.value, "value")?,
        "frame_selector": optional_string(&ctx.input.frame_selector),
    });
    let (status, body) = post(ctx.client, ctx.base_url, "/fill", ctx.token, payload).await?;
    Ok(("fill", "/fill", status, body))
}

pub(in crate::tool::browserctl) async fn type_text(ctx: &Ctx<'_>) -> Result<Outcome> {
    let payload = json!({
        "selector": require_string(&ctx.input.selector, "selector")?,
        "text": require_string(&ctx.input.text, "text")?,
        "delay_ms": ctx.input.delay_ms.unwrap_or(0),
        "frame_selector": optional_string(&ctx.input.frame_selector),
    });
    let (status, body) = post(ctx.client, ctx.base_url, "/type", ctx.token, payload).await?;
    Ok(("type", "/type", status, body))
}

pub(in crate::tool::browserctl) async fn press(ctx: &Ctx<'_>) -> Result<Outcome> {
    let payload = json!({
        "selector": require_string(&ctx.input.selector, "selector")?,
        "key": require_string(&ctx.input.key, "key")?,
        "frame_selector": optional_string(&ctx.input.frame_selector),
    });
    let (status, body) = post(ctx.client, ctx.base_url, "/press", ctx.token, payload).await?;
    Ok(("press", "/press", status, body))
}

pub(in crate::tool::browserctl) async fn text(ctx: &Ctx<'_>) -> Result<Outcome> {
    let payload = json!({
        "selector": optional_string(&ctx.input.selector),
        "frame_selector": optional_string(&ctx.input.frame_selector),
    });
    let (status, body) = post(ctx.client, ctx.base_url, "/text", ctx.token, payload).await?;
    Ok(("text", "/text", status, body))
}

pub(in crate::tool::browserctl) async fn html(ctx: &Ctx<'_>) -> Result<Outcome> {
    let payload = json!({
        "selector": optional_string(&ctx.input.selector),
        "frame_selector": optional_string(&ctx.input.frame_selector),
    });
    let (status, body) = post(ctx.client, ctx.base_url, "/html", ctx.token, payload).await?;
    Ok(("html", "/html", status, body))
}

pub(in crate::tool::browserctl) async fn click_text(ctx: &Ctx<'_>) -> Result<Outcome> {
    let payload = json!({
        "selector": optional_string(&ctx.input.selector),
        "frame_selector": optional_string(&ctx.input.frame_selector),
        "text": require_string(&ctx.input.text, "text")?,
        "timeout_ms": ctx.input.timeout_ms.unwrap_or(5_000),
        "exact": ctx.input.exact.unwrap_or(true),
        "index": ctx.input.index.unwrap_or(0),
    });
    let (status, body) = post(ctx.client, ctx.base_url, "/click/text", ctx.token, payload).await?;
    Ok(("click_text", "/click/text", status, body))
}

pub(in crate::tool::browserctl) async fn fill_native(ctx: &Ctx<'_>) -> Result<Outcome> {
    let payload = json!({
        "selector": require_string(&ctx.input.selector, "selector")?,
        "value": require_string(&ctx.input.value, "value")?,
        "frame_selector": optional_string(&ctx.input.frame_selector),
    });
    let (status, body) = post(ctx.client, ctx.base_url, "/fill/native", ctx.token, payload).await?;
    Ok(("fill_native", "/fill/native", status, body))
}

pub(in crate::tool::browserctl) async fn toggle(ctx: &Ctx<'_>) -> Result<Outcome> {
    let payload = json!({
        "selector": require_string(&ctx.input.selector, "selector")?,
        "text": require_string(&ctx.input.text, "text")?,
        "timeout_ms": ctx.input.timeout_ms.unwrap_or(5_000),
        "frame_selector": optional_string(&ctx.input.frame_selector),
    });
    let (status, body) = post(ctx.client, ctx.base_url, "/toggle", ctx.token, payload).await?;
    Ok(("toggle", "/toggle", status, body))
}
