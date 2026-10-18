//! Provider/model capability discovery for the models CLI.
//!
//! [ModelsArgs] controls provider filtering and output formatting. [execute]
//! reads configured providers through [Registry], enriches their model metadata,
//! and prints results with setup guidance on stderr when discovery is empty.

mod collect;
mod enrich;
mod guidance;
mod render;
mod types;

mod args;
mod registry;
pub mod setup;
pub use args::ModelsArgs;
pub use registry::Registry;

/// Discover models and print their capabilities with optional recovery guidance.
///
/// # Arguments
/// * `args` - Provider filter and JSON output selection.
/// * `registry` - Providers already initialized by the application.
/// # Returns
/// Unit after printing results and any guidance.
/// # Errors
/// Returns serialization errors; individual provider failures appear in results.
/// # Examples
/// ```no_run
/// # async fn example(registry: &impl codetether_models_cli::Registry) -> anyhow::Result<()> {
/// codetether_models_cli::execute(codetether_models_cli::ModelsArgs {
///     provider: None, json: true,
/// }, registry).await
/// # }
/// ```
pub async fn execute(args: ModelsArgs, registry: &impl Registry) -> anyhow::Result<()> {
    let capabilities = collect::capabilities(registry, args.provider.as_deref()).await;
    let output = if args.json {
        render::json(&capabilities)?
    } else {
        render::text(&capabilities)
    };
    println!("{output}");
    if let Some(hint) = guidance::empty_models(&capabilities) {
        eprintln!("{hint}");
    }
    Ok(())
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
