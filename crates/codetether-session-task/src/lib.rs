//! Work-item events and backward-compatible goal actions for a session journal.
//!
//! [SessionTaskTool] validates tool input and appends events to the shared goal
//! journal. Its [Host] resolves application session paths and invokes the
//! independent goal verifier for legacy completion requests.
mod budget;
mod dispatch;
mod handlers;
mod host;
mod integration;
mod params;
mod schema;
mod tool;
mod tool_impl;
pub use budget::refusal as budget_refusal;
pub use host::Host;
pub use integration::{dispatch, list_tasks};
pub use schema::value as schema_value;
pub use tool::SessionTaskTool;
