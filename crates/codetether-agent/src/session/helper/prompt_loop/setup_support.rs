//! Optional tool-router construction for the shared loop.

use crate::cognition::tool_router::{ToolCallRouter, ToolRouterConfig};

/// Creates the optional tool-call router configured by the environment.
pub(super) fn router() -> Option<ToolCallRouter> {
    ToolCallRouter::from_config(&ToolRouterConfig::from_env()).unwrap_or_else(|error| {
        tracing::warn!(error = %error, "Tool router disabled");
        None
    })
}
