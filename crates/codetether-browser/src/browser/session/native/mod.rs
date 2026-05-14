//! Native TetherScript browser backend.
//!
//! The backend stores Send-friendly session snapshots and creates
//! `BrowserPage` wrappers only inside synchronous command handlers.

mod content;
mod device;
mod dispatch;
mod dom;
mod eval;
mod fetch;
mod lifecycle;
mod navigation;
mod net;
mod screen;
mod state;
mod tabs;
mod wait;

/// Native page and runtime state.
pub(in crate::browser::session) use state::{NativePage, NativeRuntime};

use crate::browser::{BrowserCommand, BrowserError, BrowserOutput};

/// Execute a command through the native backend.
///
/// # Errors
///
/// Returns [`BrowserError`] when the session is not started or the command
/// fails in the TetherScript page model.
pub(super) async fn execute(
    session: &super::BrowserSession,
    command: BrowserCommand,
) -> Result<BrowserOutput, BrowserError> {
    dispatch::run(session, command).await
}
