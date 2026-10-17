//! Signed plugin manifests, content hashing and verified registration.
//!
//! [SigningKey] verifies [PluginManifest] signatures before [PluginRegistry]
//! accepts matching byte or file content. Process confinement lives elsewhere.
use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
mod constant_time_eq;
mod hash_bytes;
mod hash_file;
mod hash_file_async;
mod key;
mod key_debug;
mod key_from_env;
mod key_shared;
mod key_sign;
mod key_verify;
mod key_with_key;
mod manifest;
mod manifest_default;
mod registration_policy;
mod registry;
mod registry_from_env;
mod registry_get;
mod registry_is_verified;
mod registry_list;
mod registry_new;
mod registry_register;
mod registry_register_bytes;
mod registry_register_checked;
mod registry_register_file;
mod registry_signing_key;
mod registry_verify_content;
#[cfg(test)]
mod tests;
use constant_time_eq::constant_time_eq;
pub use hash_bytes::hash_bytes;
pub use hash_file::hash_file;
use hash_file_async::hash_file_async;
pub use key::SigningKey;
pub use manifest::PluginManifest;
use manifest_default::default_timeout;
use registration_policy::allow_unverified_content_registration;
pub use registry::PluginRegistry;

static GLOBAL_SIGNING_KEY: OnceLock<SigningKey> = OnceLock::new();
