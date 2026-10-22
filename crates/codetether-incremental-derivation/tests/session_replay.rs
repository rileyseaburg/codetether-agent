//! Opt-in, read-only replay of real session context through the request builder.
mod replay_support;
use anyhow::Result;
use codetether_provider_core::{ContentPart, Role};
use codetether_request_tokens::estimate_request_tokens;
use replay_support::{Request, load};

#[tokio::test]
#[ignore = "set CODETETHER_REPLAY_SESSION to a session locator; no provider calls are made"]
async fn replay_real_session_into_budgeted_provider_requests() -> Result<()> {
    let path = std::env::var("CODETETHER_REPLAY_SESSION")?;
    let (session, model, count) = load(std::path::Path::new(&path))?;
    let mut human: Vec<_> = session
        .messages
        .iter()
        .filter(|message| message.role == Role::User)
        .filter_map(|message| {
            let text = message
                .content
                .iter()
                .filter_map(|part| match part {
                    ContentPart::Text { text } => Some(text.as_str()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("\n");
            (!text.starts_with("Continue working toward the active thread goal.")
                && !text.trim().is_empty())
            .then_some(text)
        })
        .collect();
    let first = human.first().cloned().unwrap_or_default();
    human = human.into_iter().rev().take(7).collect();
    if !human.contains(&first) {
        human.push(first);
    }
    let mut report = Vec::new();
    for budget in [16_000, 32_000, 64_000, 128_000] {
        let derived = codetether_incremental_derivation::derive_incremental(
            &session,
            Request::SYSTEM,
            &[],
            budget,
        )
        .await?;
        let reference = derived
            .messages
            .iter()
            .flat_map(|message| &message.content)
            .find_map(|part| match part {
                ContentPart::Text { text } if text.starts_with("[USER REQUEST REFERENCE]") => {
                    Some(text.clone())
                }
                _ => None,
            });
        let provenance = derived.provenance.clone();
        if derived.compressed && human.len() > 1 {
            assert!(
                reference.is_some(),
                "missing user reference at budget {budget}"
            );
        }
        let mut host = Request {
            messages: derived.messages,
            model: model.clone(),
        };
        let request = codetether_context_request::attempt::build(&mut host).await?;
        let tokens = estimate_request_tokens("", &request.messages, &request.tools);
        assert!(tokens <= budget, "request exceeded {budget}: {tokens}");
        if let Some(reference) = &reference {
            for (index, text) in human.iter().enumerate() {
                if text.chars().count() <= 512 {
                    assert!(
                        reference.contains(text),
                        "human source {index} missing from reference at budget {budget}"
                    );
                }
            }
        }
        report.push(serde_json::json!({"budget":budget,"request_tokens":tokens,
            "source_messages":count,"loaded_messages":session.messages.len(),
            "request_messages":request.messages.len(),"has_user_reference":reference.is_some(),
            "provenance":provenance}));
    }
    let report = serde_json::to_string_pretty(&report)?;
    if let Ok(path) = std::env::var("CODETETHER_REPLAY_REPORT") {
        std::fs::write(path, &report)?;
    }
    eprintln!("{report}");
    Ok(())
}
