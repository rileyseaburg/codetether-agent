//! Native session-path resolution for shared goal runtime operations.
pub(in crate::session::runtime) struct NativeJournal;
impl codetether_goal_runtime::Journal for NativeJournal {
    fn for_session(session_id: &str) -> anyhow::Result<codetether_goal_core::TaskLog> {
        let log = crate::session::tasks::TaskLog::for_session(session_id)?;
        Ok(codetether_goal_core::TaskLog::at(log.path()))
    }
}
