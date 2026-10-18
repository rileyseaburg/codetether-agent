//! Native session access and working-state effects before child execution.
use crate::PromptHost;
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_input::Params;
/// Existing child lookup, message preparation and startup announcements.
/// # Implementors
/// Applications supply session access while shared functions own effect ordering.
#[async_trait]
pub trait StartupHost: PromptHost {
    /// Look up a child session and its assigned task instructions.
    fn first_session(agent_id: &str) -> Option<(Self::Session, String)>;
    /// Prepare an owner-scoped message session, preserving lookup/policy errors.
    async fn message_session(agent_id: &str, params: &Params) -> Result<Self::Session>;
    /// Attach the global bus only if the session has no bus.
    fn ensure_bus(session: &mut Self::Session);
    /// Open the child's steering channel.
    fn open_steering(agent_id: &str);
    /// Announce the working-state transition with the supplied summary.
    fn announce_working(agent_id: &str, summary: &'static str);
}
