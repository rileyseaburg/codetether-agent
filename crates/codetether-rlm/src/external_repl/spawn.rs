//! Write the initialization script and start Bun or Node.
use super::*;
use std::process::Stdio;
use tokio::process::Command;
impl ExternalRepl {
    /// Create a Bun/Node.js REPL
    ///
    /// # Arguments
    /// * `context` - Text exposed to the child process's context helpers.
    /// # Returns
    /// A child using Bun when available, otherwise Node, labeled as Bun.
    /// # Errors
    /// Returns an error if the script cannot be written or the child cannot start.
    /// # Examples
    /// ```no_run
    /// # async fn example() -> anyhow::Result<()> {
    /// use codetether_rlm::{external_repl::ExternalRepl, repl::ReplRuntime};
    /// let mut repl = ExternalRepl::spawn_bun("context").await?;
    /// assert_eq!(repl.runtime(), ReplRuntime::Bun);
    /// repl.destroy().await?;
    /// # Ok(()) }
    /// ```
    pub async fn spawn_bun(context: &str) -> Result<Self> {
        let init_script = Self::generate_bun_init(context);

        // Write init script to temp file
        let temp_dir = std::env::temp_dir().join("rlm-repl");
        tokio::fs::create_dir_all(&temp_dir).await?;
        let script_path = temp_dir.join(format!("init_{}.js", std::process::id()));
        tokio::fs::write(&script_path, init_script).await?;

        // Try bun first, fall back to node
        let runtime = if Self::is_bun_available().await {
            "bun"
        } else {
            "node"
        };

        let child = Command::new(runtime)
            .arg(&script_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        Ok(Self {
            child,
            runtime: ReplRuntime::Bun,
        })
    }
}
