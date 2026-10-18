//! Native context derivation, request construction and completion.
//!
//! [`derive_context`], [`derive_reset`] and [`derive_with_policy`] share native
//! transform and model-resolution hooks. [`derive_incremental`] uses prepared
//! summaries. Source and event types stay generic so application sessions retain
//! their existing views; request construction and retries use the shared host
//! contracts through [`build_request_with_context`] and [`complete_with_context`].
mod derive;
mod dispatch;
mod hooks;
mod incremental;
mod reset;
pub use codetether_context_dispatch::effective_policy;
pub use codetether_context_request::{
    RequestOptions, build_request_with_context, complete_with_context,
};
pub use derive::derive_context;
pub use dispatch::derive_with_policy;
pub use incremental::derive_incremental;
pub use reset::derive_reset;

/// Session operations required for native context requests and completion.
pub use codetether_context_request::Host as ContextHost;

mod compression;
pub use compression::compress_last_message_if_oversized;
