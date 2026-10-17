//! Application binding for the dedicated autonomous task tool.
mod host;
mod registry;
mod result;
pub use codetether_go_monitor::{ACTIVE_GO_RUNS, ActiveGoRun, GoRunPhase};
/// Autonomous task tool using the application's providers and Ralph executor.
pub type GoTool = codetether_go_tool::GoTool<host::AppHost>;
