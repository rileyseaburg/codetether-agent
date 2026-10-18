//! Supply application embedding resolution to the shared indexer CLI.

/// Run index creation with application-configured embedding providers.
pub async fn run(args: crate::cli::IndexArgs) -> anyhow::Result<()> {
    codetether_indexer_cli::run(args, &super::resolver::ApplicationResolver).await
}
