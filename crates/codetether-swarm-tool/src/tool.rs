//! Construction of the stateless direct-swarm tool.
use crate::runtime::Host;
use std::marker::PhantomData;

/// Execute independent tasks through an application-supplied swarm runtime.
///
/// The tool owns no runtime state; `H` supplies provider loading, worker
/// registries, prompt composition, execution and frontend observation.
/// # Examples
/// ```
/// use codetether_swarm_tool::{SwarmExecuteTool, runtime::Host};
/// use codetether_tool_core::Tool;
/// fn construct<H: Host>() -> SwarmExecuteTool<H> {
///     let tool = SwarmExecuteTool::new();
///     assert_eq!(tool.id(), "swarm_execute");
///     tool
/// }
/// ```
pub struct SwarmExecuteTool<H: Host> {
    host: PhantomData<fn() -> H>,
}

impl<H: Host> SwarmExecuteTool<H> {
    /// Create a stateless tool using the selected application runtime.
    ///
    /// # Examples
    /// ```
    /// # use codetether_swarm_tool::{SwarmExecuteTool, runtime::Host};
    /// fn tool<H: Host>() -> SwarmExecuteTool<H> { SwarmExecuteTool::new() }
    /// ```
    pub fn new() -> Self {
        Self { host: PhantomData }
    }
}

impl<H: Host> Default for SwarmExecuteTool<H> {
    fn default() -> Self {
        Self::new()
    }
}
