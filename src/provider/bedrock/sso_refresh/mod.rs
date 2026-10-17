//! Silent Bedrock SSO refresh compatibility exports.
//!
//! The refresh crate owns token exchange, expiry assessment, and secret
//! persistence. Application startup and recovery call the same entry points.
pub use codetether_bedrock_refresh::ensure_fresh;
pub(crate) use codetether_bedrock_refresh::refresh_now;
