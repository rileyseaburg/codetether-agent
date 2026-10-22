use super::replay_support::{Request, Session};
use anyhow::Result;
use codetether_provider_core::ContentPart;

pub(super) async fn project(
    source: &Session,
    model: &str,
    end: usize,
    budget: usize,
) -> Result<serde_json::Value> {
    let derived =
        codetether_incremental_derivation::derive_incremental(source, Request::SYSTEM, &[], budget)
            .await?;
    let mut host = Request {
        messages: derived.messages,
        model: model.into(),
    };
    let request = codetether_context_request::attempt::build(&mut host).await?;
    let text = request
        .messages
        .iter()
        .flat_map(|m| &m.content)
        .filter_map(|p| match p {
            ContentPart::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");
    for needle in [
        "we didnt push to forgejo yet",
        "full integration test",
        "tests/playwright/checkout/rolloverVisualProof.live.spec.ts",
    ] {
        anyhow::ensure!(
            text.contains(needle),
            "lost correction at end={end}, budget={budget}: {needle}"
        );
    }
    let tokens =
        codetether_request_tokens::estimate_request_tokens("", &request.messages, &request.tools);
    anyhow::ensure!(tokens <= budget, "over-budget request: {tokens} > {budget}");
    Ok(serde_json::json!({"end":end,"budget":budget,"tokens":tokens,"corrections_preserved":3}))
}
