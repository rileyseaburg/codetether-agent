//! Worker completion delegates durable accounting and governance to the runtime.

use super::session_context::Context;
use crate::{provider::CompletionResponse, session::Session};
use anyhow::Result;
use std::{sync::Arc, time::Instant};

pub(super) async fn complete(
    ctx: &Context<'_>,
    session: &mut Session,
) -> Result<CompletionResponse> {
    let started = Instant::now();
    let system_prompt = crate::session::runtime::goals::compose(ctx.system_prompt, &session.id);
    let temperature = Some(if super::super::prefers_temperature_one(ctx.model) {
        1.0
    } else {
        0.7
    });
    let response = super::super::complete_worker_step_with_context_fallback(
        Arc::clone(&ctx.provider),
        session,
        ctx.model,
        &system_prompt,
        ctx.tool_definitions,
        temperature,
    )
    .await?;
    crate::session::runtime::goals::record_usage(
        &session.id,
        response
            .usage
            .prompt_tokens
            .saturating_add(response.usage.completion_tokens),
        started.elapsed(),
    )
    .await?;
    Ok(response)
}
