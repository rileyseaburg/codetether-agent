//! Application version adapter for the shared default agent card.
use super::*;
impl A2AServer {
    /// Create a default agent card.
    pub fn default_card(url: &str) -> AgentCard {
        codetether_a2a_core::application_card::default_card(url, env!("CARGO_PKG_VERSION"))
    }
}
