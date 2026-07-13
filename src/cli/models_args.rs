//! Args for the `models` command.

use clap::Parser;

#[path = "models/mod.rs"]
mod models;

/// Filter options for listing available provider models.
#[derive(Parser, Debug)]
pub struct ModelsArgs {
    /// Filter by provider name
    #[arg(short, long)]
    pub provider: Option<String>,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}

impl ModelsArgs {
    /// Discovers and renders configured provider/model capabilities.
    ///
    /// # Errors
    ///
    /// Returns an error when provider initialization or JSON rendering fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_agent::cli::ModelsArgs;
    /// ModelsArgs { provider: None, json: true }.execute().await.unwrap();
    /// # });
    /// ```
    pub async fn execute(self) -> anyhow::Result<()> {
        models::execute(self).await
    }
}
