//! Claimed-task session loading, metadata assignment and repository setup.
//! [prepare] preserves resume requirements, workspace resolution, provenance
//! and timeline ordering. [Loader] and [Host] bind native storage and workspaces;
//! [SessionState] exposes metadata without owning the application session type.
mod host;
mod load;
mod prepare;
pub use host::{Host, Loader, SessionState};
pub use load::load_ids;
pub use prepare::prepare;
