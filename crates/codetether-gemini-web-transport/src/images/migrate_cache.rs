//! Optional, read-only migration from the standalone Node launcher.
use super::{ImageClient, node_cache};
use std::path::Path;

impl ImageClient {
    /// Seed a new native session from a matching private Node renewal cache.
    ///
    /// The native cache takes precedence. Missing, insecure, expired, malformed,
    /// or nonmatching Node entries are ignored. The source cache is never written.
    ///
    /// # Arguments
    /// * `directory` - The launcher's private `codetether/gemini-web` cache directory.
    ///
    /// # Returns
    /// This client, optionally seeded with validated, source-matched renewal cookies.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_gemini_web_transport::images::ImageClient;
    /// let export = std::fs::read_to_string("cookies.txt")?;
    /// let client = ImageClient::new(&export, "native-cache".into())?
    ///     .with_node_cache(std::path::Path::new("node-cache"));
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn with_node_cache(mut self, directory: &Path) -> Self {
        if self.cache.load().is_none() {
            let state = self.state.get_mut();
            if let Some((updated, cookies)) = node_cache::load(directory, &state.base) {
                state.updates = cookies;
                state.rotated_at = updated;
            }
        }
        self
    }
}
