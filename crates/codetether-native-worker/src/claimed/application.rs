//! Native claimed-session assembly using application loader and execution types.
use crate::{WorkerTaskRuntime, preparation::Loader, task_title::Title};
use codetether_a2a_worker_core::{
    outcome::TaskInner,
    task_config::TaskContext,
    task_driver::claimed::{Output, Sessions},
    task_timeline::TaskTimeline,
};
use codetether_application_registry::{Application, ToolRegistry};
use codetether_bus_core::AgentBus;
use codetether_provenance::ClaimProvenance;
use std::marker::PhantomData;
/// Claimed-session factory selecting storage, tools, workflow and swarm types.
/// # Examples
/// Select this type as the factory parameter of [`super::Executor`].
pub struct ApplicationFactory<L, T, W, E>(PhantomData<fn() -> (L, T, W, E)>);
/// Borrowed adapter created by [`ApplicationFactory`] for one worker execution.
#[doc(hidden)]
pub struct ApplicationSessions<'a, L, T, W, E> {
    runtime: &'a WorkerTaskRuntime,
    types: PhantomData<fn() -> (L, T, W, E)>,
}
impl<L, T, W, E> super::SessionFactory for ApplicationFactory<L, T, W, E>
where
    L: Loader + 'static,
    L::Session: crate::swarm_policy::Session + Title,
    T: Application,
    W: codetether_worker_tools::Host<Registry = ToolRegistry<T>> + 'static,
    E: codetether_swarm_runtime::inline::Executor<Bus = AgentBus> + 'static,
{
    type Sessions<'a> = ApplicationSessions<'a, L, T, W, E>;
    fn sessions(runtime: &WorkerTaskRuntime) -> Self::Sessions<'_> {
        ApplicationSessions {
            runtime,
            types: PhantomData,
        }
    }
}
impl<L, T, W, E> Sessions for ApplicationSessions<'_, L, T, W, E>
where
    L: Loader,
    L::Session: crate::swarm_policy::Session + Title,
    T: Application,
    W: codetether_worker_tools::Host<Registry = ToolRegistry<T>>,
    E: codetether_swarm_runtime::inline::Executor<Bus = AgentBus>,
{
    type Session = L::Session;
    async fn prepare(
        &self,
        task_id: &str,
        context: &TaskContext,
        timeline: &mut TaskTimeline,
        claim: &ClaimProvenance,
        keys: Option<serde_json::Value>,
    ) -> anyhow::Result<(Self::Session, String)> {
        crate::preparation::prepare::<L>(self.runtime, task_id, context, timeline, claim, keys)
            .await
    }
    fn preseed(&self, session: &mut Self::Session, title: &str) {
        crate::task_title::preseed_task_title(session, title);
    }
    async fn execute(
        &self,
        session: &mut Self::Session,
        context: &TaskContext,
        agent: &str,
        output: Output,
    ) -> TaskInner {
        crate::agent::execute_task_agent::<L::Session, T, W, E>(
            session,
            self.runtime,
            context,
            agent,
            output,
        )
        .await
    }
}
