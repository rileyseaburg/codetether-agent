//! Native JSON session specialization of protected legacy migration.
use crate::session::Session;
use std::path::Path;
pub(super) fn ensure(path: &Path) -> anyhow::Result<()> {
    codetether_session_store::legacy::ensure::<Session>(path)
}
