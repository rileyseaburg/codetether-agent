//! Per-user Vault login state, independent of the current shell's environment.
//!
//! CLI authentication saves a URL-bound profile. Runtime resolution prefers it
//! over stale inherited variables, except for explicit env-only/workload usage.
//! Tokens are owner-private on Unix and DPAPI-protected on Windows.

mod address;
mod auth_path;
mod capabilities;
mod crypto;
mod facts;
mod guarded;
pub mod http;
mod paths;
mod profile;
mod rejection;
mod resolve;
mod save;
mod storage;
pub mod validate;
pub use address::normalize;
pub use facts::Facts;
pub use profile::Profile;
pub use resolve::{configured, current, environment};
pub use storage::{load, save};

#[cfg(test)]
mod tests;

#[cfg(test)]
mod http_fixture;
#[cfg(test)]
mod rejection_tests;
#[cfg(test)]
mod storage_tests;
#[cfg(test)]
mod test_env;
#[cfg(test)]
mod validation_tests;
