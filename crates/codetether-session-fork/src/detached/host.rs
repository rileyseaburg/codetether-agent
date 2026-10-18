//! Native storage bindings for detached snapshots.
use codetether_provider_core::Message;
/// Field operations that distinguish detached copies from edited branches.
/// # Implementors
/// Native sessions retain their storage format and bus attachment behavior.
pub trait Session: crate::ForkSession {
    /// Replace the working transcript and classify its page sidecar.
    fn replace_transcript(&mut self, messages: Vec<Message>);
    /// Set the raw agent label without rewriting inherited provenance.
    fn set_detached_agent(&mut self);
    /// Attach the process-global bus only when no bus is set.
    fn attach_bus_if_missing(&mut self);
}
