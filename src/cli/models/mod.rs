//! Provider/model capability discovery for `codetether models`.

mod collect;
mod enrich;
mod render;
mod types;

use super::ModelsArgs;
use crate::provider::ProviderRegistry;

pub async fn execute(args: ModelsArgs) -> anyhow::Result<()> {
    let registry = ProviderRegistry::from_vault().await?;
    let capabilities = collect::capabilities(&registry, args.provider.as_deref()).await;
    let output = if args.json {
        render::json(&capabilities)?
    } else {
        render::text(&capabilities)
    };
    println!("{output}");
    Ok(())
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
