//! Plugin signature and content-verification regressions.
use super::*;
use signed_manifest::signed_manifest;
mod hash_bytes_is_deterministic;
mod plugin_registry_registers_matching_content;
mod plugin_registry_rejects_bad_signature;
mod plugin_registry_rejects_wrong_content_hash;
mod plugin_registry_requires_content_by_default;
mod sign_and_verify_roundtrip;
mod signed_manifest;
mod tampered_manifest_fails_verification;
