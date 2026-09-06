//! Payload-free process observations; EOF alone never identifies an exit cause.
use super::{failure::Failure, process::Process};
use crate::tool::ToolResult;

impl Process {
    pub(super) fn failed(&mut self, failure: Failure) -> ToolResult {
        let pid = self.child.id();
        // Observe before Drop requests termination; do not mistake our own kill
        // for the original failure. None means exit has not yet been observed.
        match self.child.try_wait() {
            Ok(Some(status)) => {
                tracing::warn!(?pid, failure_category = ?failure, exit_observed = true,
                    exit_code = ?status.code(), exit_status = %status,
                    "Desktop worker failure; process exit observed");
            }
            Ok(None) => {
                tracing::warn!(?pid, failure_category = ?failure, exit_observed = false,
                    "Desktop worker failure; process exit not observed");
            }
            Err(error) => {
                tracing::warn!(?pid, failure_category = ?failure, exit_observed = false,
                    os_error = ?error.raw_os_error(), error_kind = ?error.kind(),
                    "Desktop worker failure; exit status unavailable");
            }
        }
        failure.result()
    }
}
