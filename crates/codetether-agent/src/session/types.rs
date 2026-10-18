//! Native session type generated from the shared field and serialization schema.
//!
//! The type remains local so application-owned inherent operations are preserved.
pub use codetether_image::ImageAttachment;
pub use codetether_session_metadata::SessionMetadata;
pub use codetether_session_schema::DEFAULT_MAX_STEPS;

codetether_session_schema::declare_session!(
    pub struct Session;
);
