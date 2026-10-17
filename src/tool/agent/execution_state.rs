//! Shared per-agent execution guards and cancellation/closed-state registry.
pub(super) use codetether_collaboration_state::execution::{
    AgentRunGuard, abort, close, is_closed, is_running, register, reopen, try_start,
};
