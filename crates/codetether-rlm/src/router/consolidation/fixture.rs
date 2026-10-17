//! Loop execution with deterministic IDs and no network access.
use super::{host::Host, provider::Provider};
use crate::{
    RlmConfig,
    context_trace::ContextTrace,
    traits::{LlmMessage, LlmResponse},
};
use std::sync::Arc;
pub async fn run(
    primary: Vec<Result<LlmResponse, String>>,
    secondary: Vec<Result<LlmResponse, String>>,
    limit: usize,
    subcalls: usize,
    summary: bool,
    abort: bool,
) -> serde_json::Value {
    let first = Arc::new(Provider::new(primary));
    let second = Arc::new(Provider::new(secondary));
    let ctx = super::context::build(first.clone(), second.clone(), abort);
    let config = RlmConfig {
        max_iterations: limit,
        max_subcalls: subcalls,
        ..Default::default()
    };
    let mut host = Host::default();
    let mut messages = vec![LlmMessage::user("start".into())];
    let mut trace = ContextTrace::new(4096);
    let result = super::super::auto_loop::run(
        &ctx,
        &config,
        &mut host,
        &mut messages,
        &mut trace,
        &host_defs(),
        summary,
    )
    .await;
    serde_json::json!({"answer":result.final_answer,"iterations":result.iterations,"subcalls":result.subcalls,"aborted":result.aborted,"error":result.last_error,
        "messages":messages.iter().map(|m|serde_json::json!({"role":m.role,"text":m.text,"tool_call_id":m.tool_call_id,"tool_calls":m.tool_calls.len()})).collect::<Vec<_>>(),
        "primary":*first.calls.lock().unwrap(),"secondary":*second.calls.lock().unwrap(),"trace":trace,"host":host.calls})
}
fn host_defs() -> Vec<crate::traits::ToolDefinition> {
    use crate::router::RouterHost;
    Host::default().tool_definitions()
}
