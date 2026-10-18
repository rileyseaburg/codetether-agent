//! Native application identities, installed runtime and goal journal lookup.
use crate::session::tasks::{TaskLog, TaskState, governance_block};
pub(super) type Native = codetether_approval_review::application::Reviewer<Application>;
pub(super) struct Application;
impl codetether_approval_review::application::Application for Application {
    type Tools = crate::tool::registry_lifecycle::Native;
    type Providers = crate::provider::registry::application::Native;
    type Agent = crate::swarm::executor::LoopHost;
    fn enabled() -> Option<super::runtime::ReviewRuntime> {
        super::runtime::enabled()
    }
    async fn goal(session_id: Option<&str>) -> Option<String> {
        let log = TaskLog::for_session(session_id?).ok()?;
        let events = log.read_all().await.ok()?;
        governance_block(&TaskState::from_log(&events))
    }
}
