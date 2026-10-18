//! Supply application embedding resolution to the shared indexer CLI.

/// Run index creation with application-configured embedding providers.
pub async fn run(args: codetether_indexer_cli::IndexArgs) -> anyhow::Result<()> {
    codetether_indexer_cli::run(
        args,
        &codetether_application_providers::indexing::Resolver::<
            codetether_native_providers::ProviderRegistry,
        >::new(),
    )
    .await
}
