//! Root-filtered publication and atomic replacement of a producer snapshot.
use crate::{Diagnostics, Publisher, Snapshot, directory, uri};
use anyhow::{Context, Result};
use std::{
    io::Write,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

impl Publisher {
    /// Atomically replace the complete diagnostic state and refresh its heartbeat.
    /// # Arguments
    /// * `diagnostics` — Existing LSP cache; unrelated URIs and opaque data are omitted.
    /// # Returns
    /// Unit after the private temporary file has been flushed and renamed.
    /// # Errors
    /// Returns serialization, time, file-write, permission or atomic rename errors.
    /// # Examples
    /// See the [crate example](crate) for construction and empty-cache publication.
    pub fn publish(&self, diagnostics: &Diagnostics) -> Result<()> {
        let snapshot = Snapshot {
            schema_version: 1,
            pid: std::process::id(),
            workspace_uri: self.workspace_uri.clone(),
            server: self.server.clone(),
            updated_at_ms: SystemTime::now()
                .duration_since(UNIX_EPOCH)?
                .as_millis()
                .min(u64::MAX as u128) as u64,
            diagnostics: filtered(&self.root, diagnostics),
        };
        let mut temporary = tempfile::NamedTempFile::new_in(
            self.path.parent().context("snapshot directory missing")?,
        )?;
        directory::private_file(temporary.as_file())?;
        serde_json::to_writer(&mut temporary, &snapshot)?;
        temporary.flush()?;
        temporary.as_file().sync_all()?;
        temporary.persist(&self.path)?;
        Ok(())
    }
}

fn filtered(root: &Path, diagnostics: &Diagnostics) -> Diagnostics {
    diagnostics
        .iter()
        .filter_map(|(key, value)| {
            let key = uri::descendant(root, key)?;
            let values = value
                .iter()
                .cloned()
                .map(|mut diagnostic| {
                    diagnostic.data = None;
                    if let Some(related) = diagnostic.related_information.as_mut() {
                        related.retain(|item| {
                            uri::descendant(root, item.location.uri.as_str()).is_some()
                        });
                    }
                    diagnostic
                })
                .collect();
            Some((key, values))
        })
        .collect()
}
