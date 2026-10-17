//! PRD load operations.
use super::*;
impl Prd {
    /// # Arguments
    /// * `path` - JSON PRD file to load.
    /// # Returns
    /// The decoded PRD.
    /// # Errors
    /// Returns file-reading or JSON-decoding errors.
    /// # Examples
    /// ```no_run
    /// # async fn example() -> anyhow::Result<()> {
    /// let prd = codetether_ralph_core::Prd::load(&"prd.json".into()).await?;
    /// assert!(!prd.project.is_empty());
    /// # Ok(()) }
    /// ```

    /// Load a PRD from a JSON file
    pub async fn load(path: &PathBuf) -> anyhow::Result<Self> {
        let content = tokio::fs::read_to_string(path).await?;
        let prd: Prd = serde_json::from_str(&content)?;
        Ok(prd)
    }
}
