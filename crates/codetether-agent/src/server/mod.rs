//! HTTP Server
//!
//! Main API server for the CodeTether Agent

pub use codetether_authorization_policy as policy;
pub use codetether_http_auth as auth;
mod session_realtime;
mod session_recovery;
mod session_routes;
mod startup_host;
use codetether_task_queue as task_queue;

use crate::cli::ServeArgs;
use crate::cognition::CognitionRuntime;
use anyhow::Result;

pub use codetether_tool_discovery::{RegisterToolRequest, RegisteredTool, ToolRegistry};
pub use task_queue::{ClaimError, KnativeTask, KnativeTaskQueue};

/// HTTP request state using the application cognition runtime.
pub type AppState = codetether_server_state::AppState<CognitionRuntime>;
/// Start the HTTP server using the native runtime bindings.
pub async fn serve(args: ServeArgs) -> Result<()> {
    codetether_server_runtime::serve::<startup_host::AppHost>(
        &args.hostname,
        args.port,
        env!("CARGO_PKG_VERSION"),
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::normalize_model_reference;

    #[test]
    fn normalize_model_reference_accepts_colon_format() {
        assert_eq!(
            normalize_model_reference("openai:gpt-4o"),
            "openai/gpt-4o".to_string()
        );
    }
}

#[cfg(test)]
include!("mod_test_imports.rs");
