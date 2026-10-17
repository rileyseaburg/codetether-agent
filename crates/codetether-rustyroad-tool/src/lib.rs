//! First-party RustyRoad access through its published MCP executable.
//!
//! [RustyRoadTool] validates the selected project and environment, authorizes
//! through [Host], and owns one bounded subprocess and MCP exchange per call.
//! Protocol, process lifetime, input validation and schema have separate modules.
mod command;
mod environment;
mod host;
mod initialize;
mod invocation;
mod params;
mod process;
mod response;
mod rpc;
mod run;
mod schema;
mod session;
mod tool_impl;
pub use host::Host;
/// Adapter for the fixed RustyRoad MCP executable on PATH.
///
/// # Examples
/// ```rust
/// # fn example<H:codetether_rustyroad_tool::Host>() {
/// let _tool=codetether_rustyroad_tool::RustyRoadTool::<H>::new();
/// # }
/// ```
pub struct RustyRoadTool<H>(std::marker::PhantomData<H>);
impl<H: Host> RustyRoadTool<H> {
    /// Construct the tool using the host's identity and authorization hook.
    pub const fn new() -> Self {
        Self(std::marker::PhantomData)
    }
}
impl<H: Host> Default for RustyRoadTool<H> {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests;
