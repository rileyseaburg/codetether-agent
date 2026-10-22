use super::{host::Prepared, provider::NeverCalled};
use chrono::Utc;
use codetether_goal_core::{GoalRuntimeUpdate, GoalSourceKind, TaskEvent, TaskLog};
use codetether_provider_core::{CompletionRequest, ContentPart, Message, Role};
pub fn text(role: Role, value: &str) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text { text: value.into() }],
    }
}
pub fn prepared(prompt: String) -> Prepared {
    Prepared {
        prompt,
        model: "gpt-5.4".into(),
        provider: "openai-codex".into(),
        refresh_to: None,
        messages: vec![
            text(Role::User, "Keep the public API"),
            text(Role::Assistant, "Existing evidence"),
            text(Role::User, "continue"),
        ],
        extras: vec![text(Role::System, "Optional recalled evidence")],
        tools: vec![codetether_provider_core::ToolDefinition {
            name: "read".into(),
            description: "Read source".into(),
            parameters: serde_json::json!({"type":"object","properties":{"path":{"type":"string"}},"required":["path"]}),
        }],
    }
}
pub async fn build_both(
    mut input: Prepared,
) -> anyhow::Result<(CompletionRequest, CompletionRequest)> {
    let worker = codetether_context_request::build_request_with_context(
        std::sync::Arc::new(NeverCalled),
        &input,
        &input.model,
        &input.prompt,
        &input.tools,
        codetether_context_request::RequestOptions {
            temperature: Some(0.7),
            ..Default::default()
        },
    )
    .await?;
    let foreground = codetether_context_request::attempt::build(&mut input).await?;
    Ok((foreground, worker))
}
pub async fn consecutive() -> anyhow::Result<(CompletionRequest, CompletionRequest)> {
    let dir = tempfile::tempdir()?;
    let log = TaskLog::at(dir.path().join("goal.jsonl"));
    log.append_blocking(&TaskEvent::GoalSet {
        at: Utc::now(),
        goal_id: "g".into(),
        objective: "Finish the requested work".into(),
        success_criteria: vec![],
        forbidden: vec!["Do not deploy".into()],
        source_session_id: "s".into(),
        source_turn_id: "0".into(),
        source_text_hash: "hash".into(),
        source_kind: GoalSourceKind::UserProvided,
        confidence: 1.0,
    })?;
    let initial_prompt = codetether_goal_runtime::compose_from_log("stable rules".into(), &log);
    let (first, worker_first) = build_both(prepared(initial_prompt.clone())).await?;
    assert_eq!(
        serde_json::to_value(&first.messages)?,
        serde_json::to_value(&worker_first.messages)?
    );
    log.append_blocking(&TaskEvent::GoalRuntime(GoalRuntimeUpdate {
        at: Utc::now(),
        goal_id: "g".into(),
        objective: None,
        status: None,
        token_budget: None,
        token_delta: 750,
        elapsed_seconds: 4,
        continuation_delta: 1,
    }))?;
    let (second, worker_second) = build_both(prepared(codetether_goal_runtime::compose_from_log(
        initial_prompt,
        &log,
    )))
    .await?;
    assert_eq!(
        serde_json::to_value(&second.messages)?,
        serde_json::to_value(&worker_second.messages)?
    );
    Ok((first, second))
}
