//! Page evaluation actions: `eval` and `console_eval`.

use super::super::helpers::{optional_string, require_string};
use super::super::http::post;
use super::{Ctx, Outcome};
use anyhow::Result;
use serde_json::json;

pub(in crate::tool::browserctl) async fn eval(ctx: &Ctx<'_>) -> Result<Outcome> {
    let payload = json!({
        "expression": require_string(&ctx.input.expression, "expression")?,
        "frame_selector": optional_string(&ctx.input.frame_selector),
    });
    let (status, body) = post(ctx.client, ctx.base_url, "/eval", ctx.token, payload).await?;
    Ok(("eval", "/eval", status, body))
}

pub(in crate::tool::browserctl) async fn console_eval(ctx: &Ctx<'_>) -> Result<Outcome> {
    let payload = json!({
        "script": require_string(&ctx.input.script, "script")?,
        "frame_selector": optional_string(&ctx.input.frame_selector),
    });
    let (status, body) = post(ctx.client, ctx.base_url, "/console/eval", ctx.token, payload).await?;
    Ok(("console_eval", "/console/eval", status, body))
}
