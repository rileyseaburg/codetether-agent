//! Bind shared background dispatch to native reviewer configuration and journals.
use super::host::Native;
use crate::session::tasks::{TaskLog, TaskState, governance_block};
pub(crate) use codetether_approval_review::dispatch::Context;
impl codetether_approval_review::dispatch::Host for Native {
    fn enabled() -> Option<super::runtime::ReviewRuntime> {
        super::runtime::enabled()
    }
    async fn goal(session_id: Option<&str>) -> Option<String> {
        let log = TaskLog::for_session(session_id?).ok()?;
        let events = log.read_all().await.ok()?;
        governance_block(&TaskState::from_log(&events))
    }
}
pub(crate) fn start(
    item: &codetether_approval_queue::ApprovalSnapshot,
    context: impl FnOnce() -> Context,
) {
    codetether_approval_review::dispatch::start::<Native>(item, context);
}
