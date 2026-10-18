use super::Native;
use crate::a2a::worker::{
    TaskContext, execute_task_agent, task_execute_title::preseed_task_title, task_timeline,
};
use crate::{provenance::ClaimProvenance, session::Session};
use codetether_a2a_worker_core::{
    outcome::TaskInner,
    task_driver::claimed::{Output, Sessions},
};
impl Sessions for Native<'_> {
    type Session = Session;
    async fn prepare(
        &self,
        task_id: &str,
        context: &TaskContext,
        timeline: &mut task_timeline::TaskTimeline,
        claim: &ClaimProvenance,
        keys: Option<serde_json::Value>,
    ) -> anyhow::Result<(Session, String)> {
        codetether_native_worker::preparation::prepare::<super::super::task_session::Loader>(
            self.0, task_id, context, timeline, claim, keys,
        )
        .await
    }
    fn preseed(&self, session: &mut Session, title: &str) {
        preseed_task_title(session, title);
    }
    async fn execute(
        &self,
        session: &mut Session,
        context: &TaskContext,
        agent: &str,
        output: Output,
    ) -> TaskInner {
        execute_task_agent(session, self.0, context, agent, output).await
    }
}
