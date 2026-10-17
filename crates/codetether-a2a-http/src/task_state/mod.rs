//! Task-table operations for the JSON-RPC server.
//!
//! Records and request readers are independent of session execution. Query,
//! cancellation, activity and outcome functions operate on a supplied table.
//!
//! ```rust
//! use codetether_a2a_core::Part;
//! use codetether_a2a_http::task_state::text_of;
//! let parts = vec![Part::Text { text: "hello".into() }];
//! assert_eq!(text_of(&parts), "hello");
//! ```
use codetether_a2a_core::*;
use dashmap::DashMap;
use uuid::Uuid;
mod open_task;
pub use open_task::open_task;
mod agent_message;
pub use agent_message::agent_message;
mod response_artifact;
pub use response_artifact::response_artifact;
mod text_of;
pub use text_of::text_of;
mod blocking;
pub use blocking::blocking;
mod sender_of;
pub use sender_of::sender_of;
mod fail_empty;
pub use fail_empty::fail_empty;
mod complete;
pub use complete::complete;
mod fail;
pub use fail::fail;
mod handle_tasks_get;
pub use handle_tasks_get::handle_tasks_get;
mod handle_tasks_cancel;
pub use handle_tasks_cancel::handle_tasks_cancel;
pub mod activity;

/// Terminal message settlement with best-effort telemetry.
pub mod settlement;

/// Stream request records, response artifacts and terminal outcomes.
pub mod stream;
