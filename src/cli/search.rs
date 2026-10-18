//! Application provider and embedding policy for the shared search CLI.
use crate::cli::SearchArgs;

/// Execute search using providers initialized from the application's configuration.
/// # Errors
/// Returns provider initialization or search errors.
pub async fn execute(args: SearchArgs) -> anyhow::Result<()> {
    codetether_application_providers::cli::search::execute::<
        crate::provider::registry::application::Native,
    >(args)
    .await
}
