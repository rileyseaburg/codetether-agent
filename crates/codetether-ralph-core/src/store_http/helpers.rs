//! Request authentication and wire status labels.
use super::*;
impl HttpStore {
    pub(super) fn apply_auth(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(ref key) = self.api_key {
            req.bearer_auth(key)
        } else {
            req
        }
    }

    pub(super) fn status_str(status: RalphStatus) -> &'static str {
        match status {
            RalphStatus::Pending => "pending",
            RalphStatus::Running => "running",
            RalphStatus::Completed => "completed",
            RalphStatus::MaxIterations => "max_iterations",
            RalphStatus::Stopped => "stopped",
            RalphStatus::QualityFailed => "quality_failed",
        }
    }
}
