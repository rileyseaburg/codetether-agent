//! Native reqwest/provider stream -> fixture execution -> Role::Tool feedback.
use super::{fixture, stream};
use crate::GeminiWebProvider;
use anyhow::{Context, Result, ensure};
use codetether_provider_core::{CompletionRequest, ContentPart, Message, Provider, Role};
use serde_json::{Value, json};

pub(super) async fn exercise(
    provider: &GeminiWebProvider,
    turns: &mut Vec<Value>,
) -> Result<Value> {
    let fixture = fixture::Fixture::new();
    let mut messages = vec![Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: fixture::TASK.into(),
        }],
    }];
    for step in 0..3 {
        let request = CompletionRequest {
            messages: messages.clone(),
            tools: fixture::tools(),
            model: "gemini-web-fast".into(),
            temperature: None,
            top_p: None,
            max_tokens: Some(1024),
            stop: vec![],
        };
        let response = tokio::time::timeout(
            std::time::Duration::from_secs(90),
            provider.complete_stream(request),
        )
        .await
        .context("native provider timed out")?
        .with_context(|| format!("native inference turn {} failed", step + 1))?;
        let message = stream::collect(response).await?;
        let calls = message
            .content
            .iter()
            .filter_map(|part| match part {
                ContentPart::ToolCall {
                    id,
                    name,
                    arguments,
                    ..
                } => Some((id.clone(), name.clone(), arguments.clone())),
                _ => None,
            })
            .collect::<Vec<_>>();
        turns.push(json!({"step":step + 1,"assistant":message}));
        messages.push(message.clone());
        if step == 2 {
            ensure!(
                calls.is_empty(),
                "expected final answer, not another tool call"
            );
            let answer = message
                .content
                .iter()
                .filter_map(|part| match part {
                    ContentPart::Text { text } => Some(text.as_str()),
                    _ => None,
                })
                .collect::<String>();
            ensure!(
                answer.trim() == fixture.expected(),
                "final answer did not match hidden tool results"
            );
            return Ok(
                json!({"turns":turns,"tools_executed":2,"final_answer":answer,"matched":true}),
            );
        }
        ensure!(
            calls.len() == 1,
            "native inference turn {}: expected exactly one structured tool call, got {}",
            step + 1,
            calls.len()
        );
        let (id, name, args) = &calls[0];
        let result = fixture.execute(step, name, args)?;
        turns.last_mut().unwrap()["tool_result"] = result.clone();
        messages.push(Message {
            role: Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: id.clone(),
                content: result.to_string(),
            }],
        });
    }
    anyhow::bail!("native round trip did not reach a final answer")
}
