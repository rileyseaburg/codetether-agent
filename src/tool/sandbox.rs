//! Plugin signing, content verification, and subprocess policy checks.
//!
//! Plugin registration fails closed unless the manifest signature is valid and
//! the declared content hash matches the bytes or file supplied by the caller.
//! Subprocess execution enforces pre-spawn policy checks and reports any
//! platform fallback that is only advisory.
//!
//! Built-in tools are trusted. Third-party plugins must be registered with
//! verified content before callers should execute them.

include!("sandbox_modules.rs");

pub use sandbox_availability::{direct_fallback_env_allowed, unavailable_reason};

pub use codetether_plugin_verification::{
    PluginManifest, PluginRegistry, SigningKey, hash_bytes, hash_file,
};
pub use sandbox_execute::execute_sandboxed;

pub use codetether_sandbox_policy::{SandboxPolicy, SandboxResult};
