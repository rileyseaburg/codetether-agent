//! Host-specialized agent-management tool value.
use crate::Host;
use std::marker::PhantomData;
/// Agent-management tool with application-provided runtime effects.
/// # Examples
/// ```rust
/// use codetether_agent_tool::{AgentTool, Host};
/// fn tool<H: Host>() -> AgentTool<H> { AgentTool::new() }
/// ```
pub struct AgentTool<H>(PhantomData<fn() -> H>);
impl<H: Host> AgentTool<H> {
    /// Construct the zero-state tool using the selected host implementation.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}
impl<H: Host> Default for AgentTool<H> {
    fn default() -> Self {
        Self::new()
    }
}
