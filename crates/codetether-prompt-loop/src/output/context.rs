use codetether_bus_core::AgentBus;
use codetether_session_events::SessionEvent;
use std::sync::Arc;
use tokio::sync::mpsc;
/// Borrowed output destinations and current turn identity.
/// # Examples
/// ```
/// let mut output = String::new();
/// let _context = codetether_prompt_loop::output::Context::<()> {
///     output: &mut output, events: None, bus: None, agent: "build", turn_id: "turn", step: 1,
/// };
/// ```
pub struct Context<'a, S> {
    /// Final answer accumulator.
    pub output: &'a mut String,
    /// Optional session stream channel.
    pub events: Option<&'a mpsc::Sender<SessionEvent<S>>>,
    /// Optional agent bus.
    pub bus: Option<&'a Arc<AgentBus>>,
    /// Session agent persona.
    pub agent: &'a str,
    /// Turn correlation identifier.
    pub turn_id: &'a str,
    /// Provider step used for diagnostics.
    pub step: usize,
}
