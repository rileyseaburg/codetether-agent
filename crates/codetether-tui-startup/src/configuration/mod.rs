//! Startup access-mode, reviewer and session configuration ordering.
//!
//! [apply_policies] resolves full-auto access, consumes the loaded config and
//! applies the final auto-edit override after the session config.
mod apply;
mod full_auto;
mod host;
pub use apply::{apply, apply_policies, apply_startup};
pub use full_auto::effective_access_mode;
pub use host::Host;
