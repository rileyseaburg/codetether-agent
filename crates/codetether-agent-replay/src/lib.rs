//! Execute and replay agent tool-call batches in their original order.
//!
//! [execute] selects parallel execution only when every call is read-only and
//! permitted. [record_results] records tool usage, rendered feedback and image
//! content. Native [Host] implementations retain session policy and tool access.
mod batch;
mod host;
mod images;
mod parallel;
mod policy;
mod record;
mod sequential;
pub use batch::execute;
use codetether_agent_core::ToolUse;
use codetether_provider_core::{ContentPart, Message, Role};
use codetether_tool_core::ToolResult;
pub use host::Host;
pub use policy::{blocked, can_parallelize};
pub use record::record_results;
