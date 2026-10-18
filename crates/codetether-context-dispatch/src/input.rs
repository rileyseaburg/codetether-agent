//! Per-call context inputs consumed by exactly one selected strategy.
use crate::Source;
use codetether_context_legacy::Hooks;
use codetether_provider_core::{Provider, ToolDefinition};
use codetether_session_events::SessionEvent;
use std::sync::Arc;
use tokio::sync::mpsc;
pub(super) struct DispatchInputs<'a, H: Source, S> {
    pub session: &'a H,
    pub provider: Arc<dyn Provider>,
    pub model: &'a str,
    pub system_prompt: &'a str,
    pub tools: &'a [ToolDefinition],
    pub event_tx: Option<&'a mpsc::Sender<SessionEvent<S>>>,
    pub force_keep_last: Option<usize>,
    pub hooks: Hooks,
}
