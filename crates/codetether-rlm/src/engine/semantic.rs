//! LLM synthesis over curated evidence.

use anyhow::{Result, anyhow};

use crate::router::CrateAutoProcessContext;
use crate::traits::LlmMessage;

use super::evidence::Evidence;
use super::pack;

/// Synthesize a semantic answer from the evidence pack.
pub async fn synthesize(
    ctx: &CrateAutoProcessContext<'_>,
    query: &str,
    evidence: &Evidence,
) -> Result<String> {
    let messages = vec![
        LlmMessage {
            role: "system".into(),
            text: pack::system_prompt(),
            tool_calls: vec![],
            tool_call_id: None,
        },
        LlmMessage::user(pack::user_prompt(query, evidence)),
    ];
    let response = ctx
        .provider
        .complete(messages, vec![], &ctx.model, Some(0.2))
        .await?;
    let answer = response.text.trim();
    if answer.is_empty() {
        return Err(anyhow!("RLM engine synthesis produced no text"));
    }
    Ok(answer.to_string())
}
