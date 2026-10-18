//! Human text/image ingestion, publication and optional title generation.
//! [accept] appends one user message before publication, then generates a title
//! if needed. Native session operations are supplied by [Host].
mod accept;
mod host;
pub use accept::accept;
pub use host::Host;
