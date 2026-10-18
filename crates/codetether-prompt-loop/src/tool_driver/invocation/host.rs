//! Session effects needed by approval-aware invocation orchestration.
use crate::Call;
use codetether_tool_approval::ApprovalGate;
use codetether_tool_outcome::Outcome;
use codetether_tool_reporting::ToolTuple;
use serde_json::Value;
use std::{future::Future, time::Instant};

/// Native session and tool bindings for invocation sequencing.
/// # Implementors
/// Runners retain their storage and runtime services while the shared driver
/// orders persistence, approval, execution, warning annotation, and rendering.
pub trait Host {
    /// Enrich the original arguments with current session/workspace context.
    fn enrich(&self, call: &Call) -> Value;
    /// Persist before beginning approval, returning the execution start instant.
    fn before_tool(&mut self) -> impl Future<Output = anyhow::Result<Instant>> + Send;
    /// Apply interactive approval, or return unchanged ready input without events.
    fn gate(&self, call: &Call, input: Value) -> impl Future<Output = ApprovalGate> + Send;
    /// Execute an approved call with the existing heartbeat and runtime gates.
    fn approved(
        &self,
        call: &Call,
        input: &Value,
        started: Instant,
    ) -> impl Future<Output = ToolTuple> + Send;
    /// Render the blocked or executed result against original call context.
    fn render(
        &mut self,
        call: &Call,
        input: &Value,
        started: Instant,
        tuple: ToolTuple,
    ) -> impl Future<Output = Outcome> + Send;
}
