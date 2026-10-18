//! Compatibility exports for shared session approval grants.

pub use codetether_approval_core::session_grants::{allowed, grant};

#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub use codetether_approval_core::session_grants::reset;
