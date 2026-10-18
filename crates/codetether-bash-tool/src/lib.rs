//! Noninteractive bash execution with shared approval and sandbox policy.
//!
//! [BashTool] supports workspace defaults and timeouts. Execution modules own
//! command setup, bounded capture, audit records, telemetry and result metadata.
//!
//! ```rust
//! let _tool = codetether_bash_tool::BashTool::with_timeout(30);
//! ```
use codetether_tool_core::{Tool, ToolResult};
use std::path::PathBuf;
mod bash_auth_risk;
mod bash_interactive;
mod bash_output;
mod bash_sandbox_error_metadata;
mod bash_sandbox_metadata;
mod bash_unsandboxed_metadata;
mod call;
mod construction;
mod direct;
mod direct_auth;
mod direct_command;
mod direct_environment;
mod direct_error;
mod direct_finished;
mod direct_output;
mod direct_telemetry;
mod direct_timeout;
mod execute;
mod helpers;
mod parameters;
mod sandbox;
mod sandbox_audit;
mod sandbox_detail;
mod sandbox_error;
mod sandbox_finished;
mod tool;
mod validation;
use call::Call;
use helpers::*;
/// Executes a command with noninteractive credentials and bounded output.
///
/// # Examples
/// ```rust
/// let _tool = codetether_bash_tool::BashTool::new();
/// ```
pub struct BashTool {
    timeout_secs: u64,
    sandboxed: bool,
    default_cwd: Option<PathBuf>,
}
pub use bash_auth_risk::reason as interactive_auth_risk_reason;
pub use helpers::{codetether_wrapped_command, looks_like_auth_prompt};

#[cfg(test)]
mod native_tests;
