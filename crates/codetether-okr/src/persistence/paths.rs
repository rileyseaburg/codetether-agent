//! Repository paths operations.

use super::OkrRepository;
use std::path::PathBuf;
use uuid::Uuid;
impl OkrRepository {
    /// Get the file path for an OKR
    pub(super) fn okr_path(&self, id: Uuid) -> PathBuf {
        self.base_path
            .join("objectives")
            .join(format!("{}.json", id))
    }

    /// Get the file path for an OKR run
    pub(super) fn run_path(&self, id: Uuid) -> PathBuf {
        self.base_path.join("runs").join(format!("{}.json", id))
    }

    /// Get the directory path for OKRs
    pub(super) fn okrs_dir(&self) -> PathBuf {
        self.base_path.join("objectives")
    }

    /// Get the directory path for runs
    pub(super) fn runs_dir(&self) -> PathBuf {
        self.base_path.join("runs")
    }
}
