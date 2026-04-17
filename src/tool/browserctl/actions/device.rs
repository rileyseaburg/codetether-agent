//! Raw input device actions: mouse click and keyboard.

use super::super::helpers::{require_point, require_string};
use super::super::http::post;
use super::{Ctx, Outcome};
use anyhow::Result;
use serde_json::json;

pub(in crate::tool::browserctl) async fn mouse_click(ctx: &Ctx<'_>) -> Result<Outcome> {
    let payload = json!({
        "x": require_point(ctx.input.x, "x")?,
        "y": require_point(ctx.input.y, "y")?,
    });
    let (status, body) = post(ctx.client, ctx.base_url, "/mouse/click", ctx.token, payload).await?;
    Ok(("mouse_click", "/mouse/click", status, body))
}

pub(in crate::tool::browserctl) async fn keyboard_type(ctx: &Ctx<'_>) -> Result<Outcome> {
    let payload = json!({"text": require_string(&ctx.input.text, "text")?});
    let (status, body) = post(
        ctx.client,
        ctx.base_url,
        "/keyboard/type",
        ctx.token,
        payload,
    )
    .await?;
    Ok(("keyboard_type", "/keyboard/type", status, body))
}

pub(in crate::tool::browserctl) async fn keyboard_press(ctx: &Ctx<'_>) -> Result<Outcome> {
    let payload = json!({"key": require_string(&ctx.input.key, "key")?});
    let (status, body) = post(
        ctx.client,
        ctx.base_url,
        "/keyboard/press",
        ctx.token,
        payload,
    )
    .await?;
    Ok(("keyboard_press", "/keyboard/press", status, body))
}
