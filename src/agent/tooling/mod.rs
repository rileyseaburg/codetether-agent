//! Tool dispatch for agents.
//!
//! This module isolates tool execution, invalid-tool fallback handling, and
//! registry access for the `Agent` runtime.
//!
//! # Examples
//!
//! ```ignore
//! let has_bash = agent.has_tool("bash");
//! ```

mod dispatch;
mod registry_access;
