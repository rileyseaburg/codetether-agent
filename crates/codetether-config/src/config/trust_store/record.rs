use super::store::ProjectTrustStore;
use anyhow::Result;
use serde_json::json;
use std::path::PathBuf;

impl ProjectTrustStore {
    /// Return whether this workspace has a trust record.
    pub fn is_trusted(&self) -> bool {
        self.record_path().is_file()
    }

    /// Persist trust for this workspace.
    pub fn trust(&self) -> Result<()> {
        std::fs::create_dir_all(&self.root)?;
        let body = json!({
            "trusted": true,
            "key": self.key,
            "workspace": self.workspace,
        });
        std::fs::write(self.record_path(), serde_json::to_vec_pretty(&body)?)?;
        Ok(())
    }

    /// Remove trust for this workspace if present.
    pub fn untrust(&self) -> Result<()> {
        match std::fs::remove_file(self.record_path()) {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(err) => Err(err.into()),
        }
    }

    /// Return the trust record path for this workspace.
    pub fn record_path(&self) -> PathBuf {
        self.root.join(format!("{}.json", self.key))
    }
}
