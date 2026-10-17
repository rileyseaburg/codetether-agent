//! Prompt construction and completion result metadata.
use super::{host::Host, provider::Provider};
use crate::{RlmConfig, router::CrateAutoProcessContext};
use std::sync::Arc;
#[test]
fn preparation_snapshot() {
    let mut values = Vec::new();
    for tool in ["webfetch", "session_context"] {
        let ctx = CrateAutoProcessContext {
            tool_id: tool,
            tool_args: serde_json::json!({}),
            session_id: "fixture",
            abort: None,
            on_progress: None,
            provider: Arc::new(Provider::new(vec![])),
            model: "main".into(),
            bus: None,
            trace_id: Some(uuid::Uuid::nil()),
            subcall_provider: None,
            subcall_model: None,
            rewriter: None,
        };
        let prepared = super::super::auto_prepare::prepare(
            "fn example() {}",
            &ctx,
            &RlmConfig::default(),
            &Host::default(),
        );
        values.push(serde_json::json!({"id":prepared.trace_id,"trace":prepared.trace,"tools":prepared.tools,"summary":prepared.summary_mode,
            "tokens":prepared.input_tokens,"messages":prepared.conversation.iter().map(|m|serde_json::json!({"role":m.role,"text":m.text})).collect::<Vec<_>>()}));
    }
    println!("PREPARATION={}", serde_json::to_string(&values).unwrap());
}
