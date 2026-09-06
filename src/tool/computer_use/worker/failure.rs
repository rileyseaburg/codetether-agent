//! Stable structured worker failure categories for callers and telemetry.
use crate::tool::ToolResult;

#[derive(Clone, Copy, Debug)]
pub(super) enum Failure {
    RequestTooLarge,
    QueueTimeout,
    Spawn,
    Transport,
    Eof,
    Framing,
    Timeout,
}

impl Failure {
    pub(super) fn result(self) -> ToolResult {
        let code = match self {
            Self::RequestTooLarge => "COMPUTER_USE_WORKER_REQUEST_TOO_LARGE",
            Self::QueueTimeout => "COMPUTER_USE_WORKER_QUEUE_TIMEOUT",
            Self::Spawn => "COMPUTER_USE_WORKER_SPAWN_FAILED",
            Self::Transport => "COMPUTER_USE_WORKER_TRANSPORT_FAILED",
            Self::Eof => "COMPUTER_USE_WORKER_EXITED",
            Self::Framing => "COMPUTER_USE_WORKER_INVALID_FRAME",
            Self::Timeout => "COMPUTER_USE_WORKER_TIMEOUT",
        };
        let uncertain = matches!(
            self,
            Self::Transport | Self::Eof | Self::Framing | Self::Timeout
        );
        let message = if uncertain {
            "Action effects may be unknown; worker shadow state is lost. The action was not retried. The next request starts a new worker."
        } else {
            "Request was not sent to a worker; no action was retried."
        };
        tracing::warn!(
            error_code = code,
            action_effects_unknown = uncertain,
            "Desktop worker request failed"
        );
        ToolResult::error(format!("{code}: {message}"))
            .with_metadata("error_code", serde_json::json!(code))
            .with_metadata("action_effects_unknown", serde_json::json!(uncertain))
            .with_metadata("shadow_state_lost", serde_json::json!(uncertain))
            .with_metadata("retry_performed", serde_json::json!(false))
    }
}
