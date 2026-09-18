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
pub(crate) mod http;
mod paths;
mod profile;
mod resolve;
mod save;
mod storage;
pub(crate) mod validate;
pub(crate) use address::normalize;
pub(crate) use profile::Profile;
pub(crate) use resolve::{configured, current, environment};
pub(crate) use storage::{load, save};

#[cfg(test)]
mod tests;

#[cfg(test)]
mod http_fixture;
#[cfg(test)]
mod storage_tests;
#[cfg(test)]
mod test_env;
#[cfg(test)]
mod validation_tests;
