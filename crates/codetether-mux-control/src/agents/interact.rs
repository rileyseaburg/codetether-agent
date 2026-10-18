//! Explicitly submit input already waiting in a mux-owned TUI.

use anyhow::{Result, bail};
use serde_json::json;

use codetether_mux_client::MuxConnection;
use codetether_mux_protocol::{ClientRequest, ProgramRequest, ServerResponse};

/// Submit input already waiting in a mux-owned TUI.
///
/// # Arguments
/// * `name` - Mux session route ID.
/// # Returns
/// An acceptance envelope, or `None` when the target is absent.
/// # Errors
/// Returns discovery, missing semantic state, not-waiting or delivery errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let _ = codetether_mux_control::interact_agent("work").await?;
/// # Ok(()) }
/// ```
pub async fn interact_agent(name: &str) -> Result<Option<String>> {
    let Some(target) = super::target::load(name).await? else {
        return Ok(None);
    };
    let Some(runtime) = target.runtime() else {
        bail!("mux session '{name}' has not registered semantic TUI state");
    };
    if !runtime.needs_interaction {
        bail!("session is not waiting: {}", runtime.session_title);
    }
    let mut connection = MuxConnection::connect(&target).await?;
    let response = connection
        .request(ClientRequest::Program {
            request: ProgramRequest::Input {
                window_id: target.active_window()?,
                data: super::message::terminal_submit(),
            },
        })
        .await?;
    if !matches!(response, ServerResponse::Acknowledged) {
        bail!("mux TUI rejected the pending interaction");
    }
    Ok(Some(
        json!({"accepted": true, "delivery": "mux_server", "session_title": runtime.session_title})
            .to_string(),
    ))
}
