//! Delegation to the application's borrowed session adapter.
use super::Native;
use codetether_a2a_worker_core::{
    outcome::TaskInner,
    task_config::TaskContext,
    task_driver::claimed::{Output, Sessions},
    task_timeline::TaskTimeline,
};
use codetether_provenance::ClaimProvenance;
impl<S: Sessions, F> Sessions for Native<'_, S, F> {
    type Session = S::Session;
    async fn prepare(
        &self,
        task_id: &str,
        context: &TaskContext,
        timeline: &mut TaskTimeline,
        claim: &ClaimProvenance,
        keys: Option<serde_json::Value>,
    ) -> anyhow::Result<(Self::Session, String)> {
        self.sessions
            .prepare(task_id, context, timeline, claim, keys)
            .await
    }
    fn preseed(&self, session: &mut Self::Session, title: &str) {
        self.sessions.preseed(session, title);
    }
    async fn execute(
        &self,
        session: &mut Self::Session,
        context: &TaskContext,
        agent: &str,
        output: Output,
    ) -> TaskInner {
        self.sessions.execute(session, context, agent, output).await
    }
}
