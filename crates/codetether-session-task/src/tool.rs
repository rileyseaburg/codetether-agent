//! Construction for the session work-item tool.

use crate::Host;
use std::marker::PhantomData;

/// Session work-item tool bound to application session/verifier services.
///
/// # Examples
/// ```
/// fn construct<H:codetether_session_task::Host>()->codetether_session_task::SessionTaskTool<H> {
///     codetether_session_task::SessionTaskTool::new()
/// }
/// ```
pub struct SessionTaskTool<H: Host>(PhantomData<fn() -> H>);

impl<H: Host> Default for SessionTaskTool<H> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<H: Host> SessionTaskTool<H> {
    /// Construct a session task tool.
    pub fn new() -> Self {
        Self(PhantomData)
    }
}
