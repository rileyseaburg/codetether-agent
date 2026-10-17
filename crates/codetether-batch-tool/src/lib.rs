//! Execute independent nested tool calls against an application-owned registry.
//!
//! [BatchTool] retains a weak registry reference to avoid ownership cycles.
//! [Registry] supplies tool lookup and the authoritative per-call policy check.
//! Nested results retain their order, metadata, and image attachments.
use std::sync::{Arc, RwLock, Weak};
mod construction;
mod execute;
mod params;
pub mod policy;
mod registry;
mod schema;
mod summary;
mod summary_images;
mod tool;
pub use registry::Registry;
/// Runs nested calls concurrently using a weak reference to the owning registry.
///
/// # Examples
/// ```rust
/// # fn example<R: codetether_batch_tool::Registry>() {
/// let _tool = codetether_batch_tool::BatchTool::<R>::new();
/// # }
/// ```
pub struct BatchTool<R> {
    registry: Arc<RwLock<Option<Weak<R>>>>,
}
