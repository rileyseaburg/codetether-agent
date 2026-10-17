//! Construction and completion callback for the autonomous task tool.

use crate::Host;
use std::{marker::PhantomData, sync::Arc};
/// Autonomous OKR-to-PRD-to-Ralph tool bound to an application executor.
///
/// # Examples
/// ```
/// use codetether_go_tool::{GoTool, Host};
/// fn construct<H: Host>() -> GoTool<H> { GoTool::new() }
/// ```
pub struct GoTool<H: Host> {
    pub(crate) completion_callback: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
    host: PhantomData<fn() -> H>,
}
impl<H: Host> Default for GoTool<H> {
    fn default() -> Self {
        Self::new()
    }
}
impl<H: Host> GoTool<H> {
    /// Construct a tool without a completion callback.
    pub fn new() -> Self {
        Self {
            completion_callback: None,
            host: PhantomData,
        }
    }
    /// Construct a tool that notifies its caller after background completion.
    pub fn with_callback(cb: Arc<dyn Fn(String) + Send + Sync + 'static>) -> Self {
        Self {
            completion_callback: Some(cb),
            host: PhantomData,
        }
    }
}
