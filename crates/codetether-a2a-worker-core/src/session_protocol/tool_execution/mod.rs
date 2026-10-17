//! Worker tool invocation timing and result emission.
//!
//! [run_tool] asks [ToolExecutionHost] to resolve and authorize a tool before
//! applying the configured timeout. Successful results are emitted through
//! the shared structured sink, with a legacy text callback fallback.
use codetether_tool_core::ToolResult;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
mod host;
mod report;
mod run;
pub use host::ToolExecutionHost;
pub use run::run_tool;
