//! Bedrock endpoint selection and authentication dispatch.
//!
//! IAM signing is owned by the shared authentication crate; these methods
//! retain the provider HTTP client and bearer/SigV4 dispatch behavior.
use super::BedrockProvider;
#[allow(unused_imports)] // Retain crate-internal signing helper paths during consolidation.
pub(crate) use codetether_bedrock_auth::sigv4::{
    CanonicalUrl, canonicalize_url, hmac_sha256, sha256_hex,
};
mod dispatch;
mod urls;
