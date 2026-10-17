//! Managed child-agent checkouts and explicit integration handoff.
//!
//! [allocate] creates a child checkout from the requested repository HEAD,
//! preserving relative cwd beneath the primary checkout's managed storage.
//! [Handoff] records its identity, renders isolation guidance and adds metadata
//! to tool results. Child commits stay isolated until explicit parent review.
mod allocate;
mod attach;
mod git;
mod guidance;
mod handoff;
mod paths;
pub use allocate::allocate;
pub use handoff::Handoff;
