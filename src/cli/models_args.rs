//! Model listing arguments with native provider initialization.
pub use codetether_models_cli::ModelsArgs;
#[path = "models/mod.rs"]
mod models;

pub async fn execute(args: ModelsArgs) -> anyhow::Result<()> {
    models::execute(args).await
}
