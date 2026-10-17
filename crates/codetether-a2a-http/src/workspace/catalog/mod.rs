//! Control-plane workspace catalog filtering.
//!
//! [resolve_workspace_ids] retrieves workspace records and retains paths
//! below host-supplied roots. [ResolvedWorkspace] is the resulting identity
//! and local-path pair. No worker runtime or session state is required.
use anyhow::Result;
use reqwest::Client;
mod entries;
mod entry;
mod record;
mod request;
mod roots;
use entries::workspace_entries;
use entry::resolve_entry;
pub use record::ResolvedWorkspace;
pub use request::resolve_workspace_ids;
use roots::is_under_workspace_root;
#[cfg(test)]
mod tests;
