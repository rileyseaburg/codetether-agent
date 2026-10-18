//! Safe mux state exposed to in-process user interfaces.

use std::path::PathBuf;

pub use super::summary_window::MuxWindowSummary;
use codetether_mux_model::MuxRuntimeStatus;

/// One named mux session and the server hosting it.
///
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let summaries = codetether_mux_control::list_sessions().await?;
/// for summary in summaries { assert!(!summary.name.is_empty()); }
/// # Ok(()) }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MuxSessionSummary {
    /// Session name.
    pub name: String,
    /// Workspace directory.
    pub workspace: PathBuf,
    /// Server socket address.
    pub address: String,
    /// Server process identifier.
    pub pid: u32,
    /// Active window identifier.
    pub active_window: u64,
    /// Windows belonging to this session.
    pub windows: Vec<MuxWindowSummary>,
    /// Whether the server answered the probe.
    pub reachable: bool,
    /// Latest semantic runtime report.
    pub runtime: Option<MuxRuntimeStatus>,
}
