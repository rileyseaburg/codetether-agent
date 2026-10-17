//! Compatibility exports for presigned Bedrock bearer tokens.
//!
//! [generate_bearer_token] uses the current clock; [generate_at] accepts an
//! explicit timestamp for deterministic signing.
pub use codetether_bedrock_auth::token_gen::*;
