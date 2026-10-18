//! Native-source specialization of the combined session catalog.
use super::listing::{ApplicationSource, SessionSummary};
use anyhow::Result;
use std::path::Path;
pub async fn list_all_sessions_for_directory(dir: &Path) -> Result<Vec<SessionSummary>> {
    codetether_session_catalog::list_all_sessions_for_directory::<ApplicationSource>(dir).await
}
#[cfg(test)]
#[path = "listing_all_tests.rs"]
mod tests;

#[cfg(test)]
include!("listing_all_test_imports.rs");
