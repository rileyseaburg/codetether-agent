//! Native specialization for shared active-turn interruption.
codetether_agent_lifecycle::declare_interrupt_handler!(
    pub(in crate::tool::agent) fn handle; super::host::ApplicationHost
);
#[cfg(test)]
#[path = "interrupt/idle_tests.rs"]
mod idle_tests;
#[cfg(test)]
use codetether_agent_lifecycle as marker;
#[cfg(test)]
#[path = "interrupt/test_support.rs"]
mod test_support;
#[cfg(test)]
#[path = "interrupt/tests.rs"]
mod tests;
