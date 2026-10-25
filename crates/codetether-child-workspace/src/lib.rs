//! Managed child-agent checkouts and explicit integration handoff.
//!
//! [allocate] creates a child checkout from the requested repository HEAD,
//! preserving relative cwd beneath the primary checkout's managed storage.
//! [Handoff] records its identity, renders isolation guidance and adds metadata
//! to tool results. When a child turn completes, [Handoff::deliver] applies the
//! child branch to the parent checkout as uncommitted edits; nothing is
//! committed to or merged into the parent branch.
mod allocate;
mod attach;
mod deliver;
mod git;
mod guidance;
mod handoff;
mod paths;
pub use allocate::allocate;
pub use handoff::Handoff;
