//! Build a native-prompt request from the incident's immutable source window.
use super::replay_support::Session;
use anyhow::Result;
use codetether_agent::provider::CompletionRequest;
#[path = "host.rs"]
mod host;
#[path = "prepare.rs"]
mod prepare;
use host::Request;
pub(super) async fn build(source: &Session, model: &str) -> Result<CompletionRequest> {
    let system = codetether_agent::agent::builtin::build_system_prompt(std::path::Path::new(
        "/home/riley/spotlessbinco",
    ));
    let tools = codetether_agent::tool::ToolRegistry::with_defaults().definitions();
    let budget = std::env::var("CODETETHER_REPLAY_BUDGET")
        .ok()
        .map(|s| s.parse())
        .transpose()?
        .unwrap_or(64_000);
    let overhead = codetether_request_tokens::estimate_request_tokens(&system, &[], &tools);
    let (source, review_due) = prepare::source(source);
    let derived =
        codetether_incremental_derivation::derive_incremental(&source, &system, &tools, budget)
            .await?;
    let messages = derived.messages;
    let body = serde_json::to_string(&messages)?;
    eprintln!(
        "Evidence retained: pkce={}, cli={}, review={review_due}",
        body.contains("export async function obtainAgentPkceToken"),
        body.contains("export async function runSpotlessCliCall")
    );
    anyhow::ensure!(
        body.contains("we didnt push to forgejo yet") && body.contains("full integration test"),
        "native request lost incident corrections: budget={budget}, overhead={overhead}, history_room={}",
        budget.saturating_sub(overhead)
    );
    anyhow::ensure!(
        !review_due || body.contains("TASK CONTINUITY REVIEW"),
        "projection dropped the runtime review"
    );
    let mut request = Request {
        messages,
        system,
        tools,
        model: model.into(),
    };
    let mut request = codetether_context_request::attempt::build(&mut request).await?;
    request.max_tokens = Some(4096);
    Ok(request)
}
