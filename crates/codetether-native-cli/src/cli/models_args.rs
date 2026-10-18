//! Model listing arguments with native provider initialization.
pub use codetether_models_cli::ModelsArgs;

pub async fn execute(args: ModelsArgs) -> anyhow::Result<()> {
    codetether_application_providers::cli::models::execute::<
        codetether_native_providers::registry::application::Native,
    >(args)
    .await
}
