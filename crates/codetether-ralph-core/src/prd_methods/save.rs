//! PRD save operations.
use super::*;
impl Prd {
    /// # Arguments
    /// * `path` - JSON output file to overwrite.
    /// # Returns
    /// Unit after writing the serialized PRD.
    /// # Errors
    /// Returns JSON-encoding or file-writing errors.
    /// # Examples
    /// ```no_run
    /// # async fn example(prd: &codetether_ralph_core::Prd) -> anyhow::Result<()> {
    /// prd.save(&"prd.json".into()).await?;
    /// # Ok(()) }
    /// ```

    /// Save the PRD to a JSON file
    pub async fn save(&self, path: &PathBuf) -> anyhow::Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        tokio::fs::write(path, content).await?;
        Ok(())
    }
}
