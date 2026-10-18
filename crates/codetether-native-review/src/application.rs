//! Native provider identity, installed runtime and durable session-goal lookup.
use super::Application;
use codetether_application_registry::ToolRegistry;
use codetether_native_goals::{TaskLog, TaskState, governance_block};
impl<T, A> codetether_approval_review::application::Application for Application<T, A>
where
    T: codetether_application_registry::Application,
    A: codetether_swarm_agent::Host<Registry = ToolRegistry<T>>,
{
    type Tools = T;
    type Providers = codetether_native_providers::registry::application::Native;
    type Agent = A;
    fn enabled() -> Option<super::runtime::ReviewRuntime> {
        super::runtime::enabled()
    }
    async fn goal(session_id: Option<&str>) -> Option<String> {
        let log = TaskLog::for_session(session_id?).ok()?;
        let events = log.read_all().await.ok()?;
        governance_block(&TaskState::from_log(&events))
    }
}
