//! Host execution for shared context-policy decisions.
use super::{
    derive::derive_context, helpers::DerivedContext, incremental::derive_incremental,
    reset::derive_reset, state_header,
};
use crate::provider::{Provider, ToolDefinition};
use crate::session::{Session, SessionEvent};
use anyhow::Result;
use codetether_context_policy::derivation::{
    DerivePolicy,
    dispatch::{Execution, select},
};
use std::sync::Arc;
use tokio::sync::mpsc;
struct DispatchInputs<'a> {
    session: &'a Session,
    provider: Arc<dyn Provider>,
    model: &'a str,
    system_prompt: &'a str,
    tools: &'a [ToolDefinition],
    event_tx: Option<&'a mpsc::Sender<SessionEvent>>,
    force_keep_last: Option<usize>,
}
mod api;
pub use api::derive_with_policy;
mod execute;
mod incremental;
mod legacy;
mod reset;
