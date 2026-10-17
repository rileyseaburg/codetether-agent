//! Best-effort runtime context for crash reports.

use std::path::Path;
use std::sync::{LazyLock, RwLock};

use serde::Serialize;

static CONTEXT: LazyLock<RwLock<CrashContext>> =
    LazyLock::new(|| RwLock::new(CrashContext::default()));

/// Runtime facts captured outside the crash path.
#[derive(Clone, Debug, Default, Serialize)]
pub struct CrashContext {
    /// Current working directory or session workspace.
    pub cwd: Option<String>,
    /// Active session id, when the TUI knows it.
    pub session_id: Option<String>,
    /// Active session message count at last draw.
    pub session_messages: Option<usize>,
    /// Selected model for the active session.
    pub session_model: Option<String>,
    /// Last visible TUI status string.
    pub tui_status: Option<String>,
}

/// Record the active TUI session context.
pub fn record_tui(
    session_id: &str,
    messages: usize,
    model: Option<&str>,
    cwd: Option<&Path>,
    status: &str,
) {
    let Ok(mut ctx) = CONTEXT.write() else { return };
    ctx.cwd = cwd.map(|p| p.display().to_string());
    ctx.session_id = Some(session_id.to_string());
    ctx.session_messages = Some(messages);
    ctx.session_model = model.map(str::to_string);
    ctx.tui_status = Some(status.to_string());
}

/// Return the latest recorded crash context.
pub fn snapshot() -> CrashContext {
    CONTEXT.read().map(|ctx| ctx.clone()).unwrap_or_default()
}
