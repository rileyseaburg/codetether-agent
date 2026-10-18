//! Native session and steering adapters for shared child message delivery.
mod session;
use crate::session::Session;
use crate::session::helper::steering::{self, SteeringInput};
use crate::tool::agent::store;
use async_trait::async_trait;
use codetether_agent_communication::Host;
use codetether_provider_core::Message;
/// Application registry and steering services.
pub struct ApplicationHost;
#[async_trait]
impl Host for ApplicationHost {
    type Session = Session;
    fn resolve(target: &str, owner: Option<&str>) -> Option<String> {
        store::scope::resolve_for_parent(target, owner).map(|(id, _)| id)
    }
    fn session(id: &str) -> Option<Session> {
        store::get(id).map(|entry| entry.session)
    }
    fn push(id: &str, message: &str) -> bool {
        let input = SteeringInput::new(message.into(), Vec::new());
        steering::push(id, input)
    }
    fn add_human_message(session: &mut Session, message: Message) {
        session::add_human_message(session, message);
    }
    async fn save(session: &Session) -> anyhow::Result<()> {
        session::save(session).await
    }
    fn update_session(id: &str, session: Session) {
        store::update_session(id, session);
    }
}
