pub(super) struct TaskExecutionResult {
    pub(super) status: &'static str,
    pub(super) output: Option<String>,
    pub(super) error_message: Option<String>,
    pub(super) session_id: Option<String>,
}

impl TaskExecutionResult {
    pub(super) fn completed(output: String) -> Self {
        Self::new("completed", Some(output), None, None)
    }

    pub(super) fn failed(error_message: String) -> Self {
        Self::new("failed", None, Some(error_message), None)
    }

    pub(super) fn with_session_id(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }

    pub(super) fn new(
        status: &'static str,
        output: Option<String>,
        error_message: Option<String>,
        session_id: Option<String>,
    ) -> Self {
        Self {
            status,
            output,
            error_message,
            session_id,
        }
    }
}
