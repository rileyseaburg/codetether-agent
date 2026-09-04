//! Swarm and standard-session execution result mapping.

pub(super) fn map_swarm_result(
    text: String,
    success: bool,
    session_id: String,
) -> (&'static str, Option<String>, Option<String>, Option<String>) {
    if success {
        ("completed", Some(text), None, Some(session_id))
    } else {
        (
            "failed",
            Some(text),
            Some("Swarm execution completed with failures".into()),
            Some(session_id),
        )
    }
}

/// Maps a standard worker session outcome onto the task release tuple.
///
/// A run that exhausted its step budget without the agent signalling
/// completion is reported as `failed` with a diagnostic; reporting it as
/// `completed` let the Forgejo/Temporal orchestrator treat unfinished work
/// as done (riley/spotlessbinco#5847).
pub(super) fn map_session_result(
    text: String,
    session_id: String,
    budget_exhausted: bool,
    max_steps: usize,
) -> (&'static str, Option<String>, Option<String>, Option<String>) {
    if budget_exhausted {
        return (
            "failed",
            Some(text),
            Some(format!(
                "step budget ({max_steps}) exhausted before the agent finished"
            )),
            Some(session_id),
        );
    }
    ("completed", Some(text), None, Some(session_id))
}

#[cfg(test)]
mod tests {
    use super::map_session_result;

    #[test]
    fn finished_run_is_completed() {
        let (status, text, error, session) =
            map_session_result("done".into(), "s1".into(), false, 200);
        assert_eq!(status, "completed");
        assert_eq!(text.as_deref(), Some("done"));
        assert!(error.is_none());
        assert_eq!(session.as_deref(), Some("s1"));
    }

    #[test]
    fn budget_exhausted_run_is_failed_with_diagnostic() {
        let (status, text, error, _) =
            map_session_result("partial".into(), "s1".into(), true, 50);
        assert_eq!(status, "failed");
        assert_eq!(text.as_deref(), Some("partial"));
        assert!(error.unwrap().contains("step budget (50)"));
    }
}
