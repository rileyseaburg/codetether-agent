//! Compatibility exports for shared session approval grants.

pub use codetether_approval_core::session_grants::{allowed, grant};

#[cfg(test)]
pub(crate) use codetether_approval_core::session_grants::reset;
