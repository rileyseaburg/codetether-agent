use super::{fixtures, provider};
use codetether_agent::session::Session;
use codetether_context_policy::{
    derivation::{DerivePolicy, options::RequestOptions},
    request::from_derived,
};
use codetether_goal_core::TaskLog;
use codetether_provider_core::Role;
use std::sync::Arc;

pub async fn verify(
    session: &Session,
    log: &TaskLog,
    model: &str,
    force: Option<usize>,
    provider: Arc<provider::RejectingProvider>,
    expected: &str,
) -> anyhow::Result<usize> {
    let prompt = codetether_goal_runtime::compose_from_log("fixed system instructions".into(), log);
    let derived = codetether_context_dispatch::derive_with_policy::<_, ()>(
        session,
        provider,
        model,
        &prompt,
        &[],
        None,
        DerivePolicy::Incremental {
            budget_tokens: 4_000,
        },
        force,
        provider::hooks(),
    )
    .await?;
    assert!(
        derived.messages.len() < session.messages.len(),
        "fixture must exercise context reduction"
    );
    let rendered = serde_json::to_string(&derived.messages[0])?;
    for requirement in [
        "Keep all evidence 🦀.",
        "Never deploy.",
        "Preserve API.",
        expected,
    ] {
        assert!(rendered.contains(requirement));
    }
    assert!(!rendered.contains("Use red."));
    let first = codetether_context_request::finish(
        from_derived(
            &prompt,
            derived.messages.clone(),
            model,
            &[],
            RequestOptions {
                max_tokens: Some(1024),
                ..Default::default()
            },
        ),
        vec![],
    )?;
    fixtures::charge(log, 17)?;
    let fresh = codetether_goal_runtime::compose_from_log(prompt, log);
    let second = codetether_context_request::finish(
        from_derived(
            &fresh,
            derived.messages,
            model,
            &[],
            RequestOptions {
                max_tokens: Some(1024),
                ..Default::default()
            },
        ),
        vec![],
    )?;
    assert_eq!(
        serde_json::to_value(&first.messages[0])?,
        serde_json::to_value(&second.messages[0])?
    );
    let differences = first
        .messages
        .iter()
        .zip(&second.messages)
        .filter(|(a, b)| serde_json::to_value(a).unwrap() != serde_json::to_value(b).unwrap())
        .count();
    assert_eq!(
        differences, 1,
        "only runtime accounting changes between requests"
    );
    assert_eq!(second.messages[0].role, Role::System);
    if model.starts_with("gpt") {
        let a = codetether_codex_protocol::CodexProtocol::build_http_responses_body(
            &first,
            model.into(),
            None,
            None,
        );
        let b = codetether_codex_protocol::CodexProtocol::build_http_responses_body(
            &second,
            model.into(),
            None,
            None,
        );
        assert_eq!(a["instructions"], b["instructions"]);
        assert_eq!(a["prompt_cache_key"], b["prompt_cache_key"]);
    } else {
        let (a, _) = codetether_provider_anthropic::encode_messages(&first.messages, true);
        let (b, _) = codetether_provider_anthropic::encode_messages(&second.messages, true);
        assert_eq!(a, b);
    }
    Ok(first.messages.len())
}
