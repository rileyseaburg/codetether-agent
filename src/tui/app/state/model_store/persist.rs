//! Load and persist operations for [`ModelStore`].

use super::store::ModelStore;

impl ModelStore {
    /// Load the cached model list. Returns an empty vec on any failure.
    pub fn load(&self) -> Vec<String> {
        let Ok(bytes) = std::fs::read(self.path()) else {
            return Vec::new();
        };
        serde_json::from_slice::<Vec<String>>(&bytes).unwrap_or_default()
    }

    /// Persist the model list. Errors are logged and otherwise ignored.
    pub fn store(&self, models: &[String]) {
        if let Some(parent) = self.path().parent() {
            if let Err(error) = std::fs::create_dir_all(parent) {
                tracing::debug!(%error, "failed to create model store dir");
                return;
            }
        }
        match serde_json::to_vec(models) {
            Ok(bytes) => {
                if let Err(error) = std::fs::write(self.path(), bytes) {
                    tracing::debug!(%error, "failed to write model store");
                }
            }
            Err(error) => tracing::debug!(%error, "failed to serialize model store"),
        }
    }
}
