//! Resolve one named mux target from the authenticated local registry.

use anyhow::Result;

use crate::mux::registry::MuxRecord;

/// Load the authenticated registry record matching `name`.
///
/// # Errors
///
/// Returns an error when the mux registry cannot be scanned.
pub(super) async fn load(name: &str) -> Result<Option<MuxRecord>> {
    Ok(crate::mux::registry::list()
        .await?
        .into_iter()
        .find(|record| record.name == name))
}
