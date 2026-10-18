//! Native CLI execution without provider discovery or an agent loop.
use anyhow::{Result, ensure};
use codetether_provider_bonsai::{BonsaiConfig, BonsaiProvider};
use codetether_provider_core::Provider;
/// Run requests directly, streaming text and printing separate phase timings to stderr.
/// # Arguments
/// * `args` — Prompt and generation options.
/// # Returns
/// Returns after generation or a native-runtime error; no remote fallback is attempted.
/// # Errors
/// Propagates missing-feature, checkpoint, CUDA, sampling, or output errors.
/// # Examples
/// ```text
/// codetether bonsai --prompt "Hello" --repeat 2
/// ```
pub async fn execute(args: super::BonsaiArgs) -> Result<()> {
    ensure!((1..=10).contains(&args.repeat), "repeat must be 1..=10");
    let provider = BonsaiProvider::new(BonsaiConfig::from_environment()?)?;
    for run in 1..=args.repeat {
        let request = codetether_provider_core::request_identity::inject(
            super::request::from_args(&args),
            provider.name(),
            codetether_provider_bonsai::MODEL,
        );
        let mut stream = provider.complete_stream(request).await?;
        super::stream::consume(&mut stream).await?;
        println!();
        if let Some(timing) = provider.last_timing() {
            eprintln!(
                "{}",
                serde_json::json!({"run":run,"timing":timing,"decode_tps":timing.decode_tps()})
            );
        }
    }
    Ok(())
}
