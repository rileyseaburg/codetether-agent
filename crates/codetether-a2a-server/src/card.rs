//! Application version adapter for the shared default agent card.
use super::*;
impl<H: Application> A2AServer<H> {
    /// Create a default agent card.
    /// # Arguments
    /// `url` is the externally reachable A2A endpoint.
    /// # Returns
    /// The standard CodeTether card with the application's package version.
    /// # Examples
    /// ```no_run
    /// # fn example<H: codetether_a2a_server::Application>() {
    /// let _card = codetether_a2a_server::A2AServer::<H>::default_card("http://localhost:8000");
    /// # }
    /// ```
    pub fn default_card(url: &str) -> AgentCard {
        codetether_a2a_core::application_card::default_card(url, H::version())
    }
}
